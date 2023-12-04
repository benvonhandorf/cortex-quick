// #![cfg_attr(not(test), no_std)]
#![no_std]
#![no_main]

mod keyboard_matrix;
mod kib_board;

// #[cfg(not(feature = "use_semihosting"))]
// use panic_halt as _;
// #[cfg(feature = "use_semihosting")]
// use panic_semihosting as _;

use kib_board as bsp;

use bsp::hal;
use bsp::pac;

use bsp::entry;
use cortex_m::asm;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::*;
use hal::timer::*;
use pac::{CorePeripherals, Peripherals};

use smart_leds::{hsv::RGB8, SmartLedsWrite};
use ws2812_timer_delay as ws2812;

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let gclk0 = clocks.gclk0();
    let tc12 = &clocks.tc1_tc2(&gclk0).unwrap();
    let mut led_timer = TimerCounter::tc1_(tc12, peripherals.TC1, &mut peripherals.PM);
    led_timer.start(MegaHertz::MHz(3).into_duration());

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut output_pin = pins.int.into_push_pull_output();

    let mut keyboard_matrix = keyboard_matrix::KeyboardMatrix::new(
        pins.row_a.into_push_pull_output(),
        pins.row_b.into_push_pull_output(),
        pins.row_c.into_push_pull_output(),
        pins.row_d.into_push_pull_output(),
        pins.row_e.into_push_pull_output(),
        pins.col_n.into_pull_down_input(),
        pins.col_m.into_pull_down_input(),
        pins.col_o.into_pull_down_input(),
        pins.col_p.into_pull_down_input(),
        pins.col_q.into_pull_down_input(),
    );

    loop {
        let result = keyboard_matrix.scan();

        if result.pressed_count > 0 || result.released_count > 0 {
            rprintln!("Key state: {}", result.depressed_count);
        }

        if result.depressed_count > 0 {
            output_pin.toggle().ok();
        } 
    }
}
