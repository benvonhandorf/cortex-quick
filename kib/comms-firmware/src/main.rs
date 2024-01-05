#![no_std]
#![no_main]

mod kib_board;
// mod comms;

use hal::sercom::Sercom;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use kib_board as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use core::cell::RefCell;

use cortex_m::interrupt as interrupt_helpers;
use cortex_m::peripheral::NVIC;
use pac::interrupt;
use pac::{CorePeripherals, Peripherals};

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::*;
use hal::timer::*;

use rtt_target::rprintln;

use rtt_target::rtt_init_print;

use comms::BusStatus;

static mut output_pin: Option<
    hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<hal::gpio::PushPull>>,
> = None;

use hal::sercom::i2c;

const ADDRESS: u8 = 0x22;

static mut DATA: [u8; 4] = [0u8; 4];
static SERCOM_REF: interrupt_helpers::Mutex<RefCell<Option<pac::SERCOM0>>> =
    interrupt_helpers::Mutex::new(RefCell::new(None));
// static SERCOM_REF: RefCell<Option<pac::SERCOM0>> = RefCell::new(None);
// static SERCOM_REF: Option<&mut pac::SERCOM0> = None;

static BUS_STATUS: interrupt_helpers::Mutex<RefCell<Option<BusStatus>>> =
    interrupt_helpers::Mutex::new(RefCell::new(None));

static mut INFO: Option<u32> = None;

#[interrupt]
fn SERCOM0() {
    unsafe {
        output_pin.as_mut().unwrap().toggle().ok();
    }

    interrupt_helpers::free(|cs| unsafe {
        if let Some(sercom0) = SERCOM_REF.borrow(cs).borrow_mut().as_mut() {
            if let Some(bus_status) = BUS_STATUS.borrow(cs).borrow_mut().as_mut() {
                let i2cs0 = sercom0.i2cs();

                let intflag = i2cs0.intflag.read();
                let status = i2cs0.status.read();

                if intflag.amatch().bit_is_set() {
                    bus_status.addr(status.dir().bit_is_set());

                    i2cs0.intflag.write(|w| w.amatch().set_bit());
                }

                if intflag.drdy().bit_is_set() {
                    if bus_status.is_reading() {
                        if let data = bus_status.read_data().unwrap() {
                            //Assign the next data byte, await an ACK (for more data) or NAC/Stop.
                            i2cs0.data.write(|w| w.data().bits(data));

                            i2cs0.ctrlb.write(|w| w.cmd().bits(0x3));
                        } else {
                            //No more data to transmit.  Wait for a S/Sr.
                            i2cs0.ctrlb.write(|w| w.cmd().bits(0x2));
                        }                        
                    } else {
                        //Reading the data clears the interrupt
                        let data = i2cs0.data.read().bits();

                        bus_status.write_data(data);
                    }

                    // i2cs0.intflag.write(|w| w.drdy().set_bit());
                }

                if intflag.prec().bit_is_set() {
                    i2cs0.intflag.write(|w| w.prec().set_bit());

                    bus_status.stop();
                }

                if intflag.error().bit_is_set() {
                    i2cs0.intflag.write(|w| w.error().set_bit());
                }
            }
        }
    });
}

fn configure_sercom0(sercom0: &mut pac::SERCOM0) {
    let i2cs0 = sercom0.i2cs();

    i2cs0.ctrla.write(|w| {
        unsafe {
            w.mode().i2c_slave();
            w.lowtouten().set_bit();
            w.speed().bits(0x00);
            // w.sclsm().set_bit();
            // w.sdahold().bits(0x01);
        }
        w
    });

    i2cs0.ctrlb.write(|w| {
        unsafe {
            w.amode().bits(0x00);
            // w.aacken().set_bit(); //Setting this prevents the AMATCH interrupt from firing
            w.smen().set_bit(); //Setting this causes data to be acked when read
        }
        w
    });

    i2cs0.addr.write(|w| {
        unsafe {
            w.tenbiten().clear_bit();
            w.addr().bits(ADDRESS.into());
            w.addrmask().bits(0x00); //Set bits are ignored for address matching
            w.gencen().clear_bit();
        }
        w
    });

    i2cs0.intenset.write(|w| {
        w.error().set_bit();
        w.amatch().set_bit();
        w.drdy().set_bit();
        w.prec().set_bit();

        w
    });

    i2cs0.ctrla.modify(|_, w| w.enable().set_bit());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals: Peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let gclk0 = clocks.gclk0();
    let tc12 = &clocks.tc1_tc2(&gclk0).unwrap();
    let mut led_timer = TimerCounter::tc1_(tc12, peripherals.TC1, &mut peripherals.PM);
    led_timer.start(MegaHertz::MHz(5).into_duration());

    let pins = bsp::Pins::new(peripherals.PORT);

    rprintln!("Starting configuration");

    //Configure I2C
    let sercom0_clock = &clocks.sercom0_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(pins.sda, pins.scl);

    let mut comms_status = BusStatus::new();

    let mut sercom0 = peripherals.SERCOM0;

    sercom0.enable_apb_clock(&peripherals.PM);

    configure_sercom0(&mut sercom0);

    interrupt_helpers::free(|cs| {
        BUS_STATUS.borrow(cs).replace(Some(comms_status));
        SERCOM_REF.borrow(cs).replace(Some(sercom0));
    });

    unsafe {
        output_pin = Some(pins.int.into_push_pull_output());
        core.NVIC.set_priority(interrupt::SERCOM0, 1);
        NVIC::unmask(interrupt::SERCOM0);
    }

    rprintln!("Configuration complete");

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let delta_t_ms = 3;

    loop {
        interrupt_helpers::free(|cs| unsafe {
            if let Some(comms_status) = BUS_STATUS.borrow(cs).borrow_mut().as_mut() {
                if let Some(command) = comms_status.process() {
                    rprintln!("Command: {} Reg: {:#04x} data: {:#04x} {:#04x} {:#04x}", command.read_direction, command.register, command.data[0], command.data[1], command.data[2]);
                }
            }
        });

    }
}
