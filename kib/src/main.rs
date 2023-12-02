#![no_std]
#![no_main]

mod kib_board;
mod keyboard_matrix;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use kib_board as bsp;

use bsp::hal;
use bsp::pac;

use cortex_m::asm;
use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::timer::*;
use hal::time::*;
use pac::{CorePeripherals, Peripherals};

use ws2812_timer_delay as ws2812;
use smart_leds::{hsv::RGB8, SmartLedsWrite};

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

    let gclk0 = clocks.gclk0();
    let tc12 = &clocks.tc1_tc2(&gclk0).unwrap();
    let mut led_timer = TimerCounter::tc1_(tc12, peripherals.TC1, &mut peripherals.PM);
    led_timer.start(MegaHertz::MHz(3).into_duration());

    let pins = bsp::Pins::new(peripherals.PORT);

    let mut keyboard_matrix = keyboard_matrix::KeyboardMatrix::new(
        pins.row_a.into_push_pull_output(),
        pins.col_p.into_floating_input(),
    );

    let result = keyboard_matrix.scan();

    const LED_VALUE_COUNT : usize = 21;

    let mut led_data : [RGB8; LED_VALUE_COUNT] = [RGB8::default(); LED_VALUE_COUNT];

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let led_data_pin = pins.led_data.into_push_pull_output();

    let mut led_string = ws2812::Ws2812::new(led_timer, led_data_pin);

    let color_sequence = [
        RGB8 { r:128, g:128,b: 55,},
        RGB8 { r:168, g:125,b: 61,},
        RGB8 { r:203, g:116,b: 64,},
        RGB8 { r:232, g:102,b: 64,},
        RGB8 { r:250, g:84,b: 61,},
        RGB8 { r:255, g:64,b: 55,},
        RGB8 { r:250, g:44,b: 47,},
        RGB8 { r:232, g:26,b: 37,},
        RGB8 { r:203, g:12,b: 27,},
        RGB8 { r:168, g:3,b: 17,},
        RGB8 { r:128, g:0,b: 9,},
        RGB8 { r:88, g:3,b: 3,},
        RGB8 { r:53, g:12,b: 0,},
        RGB8 { r:24, g:26,b: 0,},
        RGB8 { r:6, g:44,b: 3,},
        RGB8 { r:0, g:64,b: 9,},
        RGB8 { r:6, g:84,b: 17,},
        RGB8 { r:24, g:102,b: 27,},
        RGB8 { r:53, g:116,b: 37,},
        RGB8 { r:88, g:125,b: 47,},
    ];
    let mut infinite_color_sequence = color_sequence.iter().cycle();

    let delay_time = 200u8;
    loop {
        for i in (1..LED_VALUE_COUNT).rev() {
            led_data[i] = led_data[i - 1];
        }

        led_data[0] = infinite_color_sequence.next().unwrap().clone();

        led_string.write(led_data.iter().cloned()).unwrap();

        delay.delay_ms(delay_time);
    }
}