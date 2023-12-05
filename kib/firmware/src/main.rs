#![no_std]
#![no_main]

mod kib_board;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

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

use rtt_target::rprint;
use ws2812_timer_delay as ws2812;

use rtt_target::{rtt_init_print, rprintln};

use keyboard_matrix::KeyboardMatrix;
use illuminator::Illuminator;

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
    led_timer.start(MegaHertz::MHz(5).into_duration());

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut output_pin = pins.int.into_push_pull_output();

    let mut delay = Delay::new(core.SYST, &mut clocks);


    let mut keyboard_matrix = KeyboardMatrix::new(
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

    let mut led_data_pin = pins.led_data.into_push_pull_output();
    led_data_pin.set_drive_strength(true);

    let mut led_strand = ws2812::Ws2812::new(led_timer, led_data_pin);

    let mut illuminator = Illuminator::new(&mut led_strand);

    loop {
        let keystate = keyboard_matrix.scan(&mut delay);

        // print_keystate(&keystate);

        illuminator.decay();

        illuminator.update(&keystate);

        illuminator.render();

        output_pin.toggle().ok();
    }
}

fn print_keystate(keystate: &keyboard_matrix::KeyboardState) {
    if keystate.pressed_count > 0 || keystate.released_count > 0 {
        rprint!("Keys {}: ", keystate.depressed_count);
        for i in 0..21 {
            if keystate.state[i] {
                rprint!("{} ", i);
            }
        }

        rprintln!("");
    }
}