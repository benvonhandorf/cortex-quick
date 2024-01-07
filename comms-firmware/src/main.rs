#![no_std]
#![no_main]

mod kib_board;
mod i2c_peripheral;

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

// static mut output_pin: Option<
//     hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<hal::gpio::PushPull>>,
// > = None;

use hal::sercom::i2c;

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

    let output_pin = Some(pins.int.into_push_pull_output());

    i2c_peripheral::configure_bus_status(output_pin);

    i2c_peripheral::configure_sercom0(sercom0, 0x22);

    unsafe {
        core.NVIC.set_priority(interrupt::SERCOM0, 1);
        NVIC::unmask(interrupt::SERCOM0);
    }

    rprintln!("Configuration complete");

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let delta_t_ms = 3;

    let mut read_data: [u8; 20] = [0; 20];
    let mut count = 0;

    loop {
        interrupt_helpers::free(|cs| unsafe {
            if let Some(comms_status) = i2c_peripheral::BUS_STATUS.borrow(cs).borrow_mut().as_mut() {
                if let Some(command) = comms_status.process() {
                    rprintln!("Command: {} Reg: {:#04x} data: {:#04x} {:#04x} {:#04x}", command.read_direction, command.register, command.data[0], command.data[1], command.data[2]);

                    read_data[0] = command.register;
                    read_data[1] = count.into();

                    comms_status.provide_data(command.register, &read_data, 2);
                    count += 1;
                }
            }
        });

    }
}
