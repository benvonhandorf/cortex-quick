#![no_std]
#![no_main]

mod rib_board;

// use led_matrix;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use rib_board as bsp;
// use samd10_bare as bsp;
use bsp::hal;
use bsp::pac;
use atsamd_hal::gpio::{
    DynPin
};

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);

    let mut matrix_a : DynPin = pins.matrix_a.into();
    let mut matrix_b: DynPin = pins.matrix_b.into();
    let mut matrix_c: DynPin = pins.matrix_c.into();
    let mut matrix_d: DynPin = pins.matrix_d.into();
    let mut matrix_e: DynPin = pins.matrix_e.into();

    matrix_d.into_floating_disabled();
    matrix_e.into_floating_disabled();

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let delay_time = 200u8;
    loop {
        delay.delay_ms(delay_time);
        matrix_a.into_push_pull_output();
        matrix_a.set_high();
        matrix_b.into_push_pull_output();
        matrix_b.set_low();
        matrix_c.into_floating_disabled();

        delay.delay_ms(delay_time);
        matrix_a.into_push_pull_output();
        matrix_a.set_high();
        matrix_b.into_floating_disabled();
        matrix_c.into_push_pull_output();
        matrix_c.set_low();

        delay.delay_ms(delay_time);
        matrix_a.into_push_pull_output();
        matrix_a.set_low();
        matrix_b.into_floating_disabled();
        matrix_c.into_push_pull_output();
        matrix_c.set_high();

        delay.delay_ms(delay_time);
        matrix_a.into_push_pull_output();
        matrix_a.set_low();
        matrix_b.into_push_pull_output();
        matrix_b.set_high();
        matrix_c.into_floating_disabled();
    }
}