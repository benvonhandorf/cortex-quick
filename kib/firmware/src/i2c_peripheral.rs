
use crate::kib_board as bsp;

use bsp::hal;
use bsp::pac;

use cortex_m::interrupt as interrupt_helpers;
use core::cell::RefCell;
use pac::interrupt;

use comms::BusStatus;

use atsamd_hal::prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

static OUTPUT_PIN: interrupt_helpers::Mutex<RefCell<Option<hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<hal::gpio::PushPull>>>>> =
interrupt_helpers::Mutex::new(RefCell::new(None));

static SERCOM_REF: interrupt_helpers::Mutex<RefCell<Option<pac::SERCOM0>>> =
    interrupt_helpers::Mutex::new(RefCell::new(None));

pub static BUS_STATUS: interrupt_helpers::Mutex<RefCell<Option<BusStatus>>> =
    interrupt_helpers::Mutex::new(RefCell::new(None));

#[interrupt]
fn SERCOM0() {
    interrupt_helpers::free(|cs| unsafe {
        if let Some(output_pin) = OUTPUT_PIN.borrow(cs).borrow_mut().as_mut() {
            output_pin.toggle().ok();
        }

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
                        let data = bus_status.read_data();
                        //Assign the next data byte, await an ACK (for more data) or NAC/Stop.
                        i2cs0.data.write(|w| w.data().bits(data));

                        i2cs0.ctrlb.write(|w| w.cmd().bits(0x3));
                        // Not clear to me what case this is meant to handle or how we should get here.
                        // } else {
                        //     //No more data to transmit.  Wait for a S/Sr.
                        //     i2cs0.ctrlb.write(|w| w.cmd().bits(0x2));
                        // }                        
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


pub fn configure_sercom0(sercom0: pac::SERCOM0, address: u8) {
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
            w.addr().bits(address.into());
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

    interrupt_helpers::free(|cs| {
        SERCOM_REF.borrow(cs).replace(Some(sercom0));
    });
}

pub fn configure_bus_status(output_pin: Option<hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<hal::gpio::PushPull>>>) {
    interrupt_helpers::free(|cs| {
        BUS_STATUS.borrow(cs).replace(Some(BusStatus::new()));

        OUTPUT_PIN.borrow(cs).replace(output_pin);
    });
}