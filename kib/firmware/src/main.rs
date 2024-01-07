#![no_std]
#![no_main]

mod kib_board;
mod i2c_peripheral;
// mod ws2812_custom;

use core::borrow::Borrow;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use kib_board as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use cortex_m::interrupt as interrupt_helpers;
use cortex_m::peripheral::NVIC;
use pac::interrupt;
use pac::{CorePeripherals, Peripherals};

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::*;
use hal::timer::*;
use hal::sercom::i2c;
use hal::sercom::Sercom;

// use ws2812_custom as ws2812;
use ws2812_timer_delay as ws2812;

use keyboard_matrix::KeyboardMatrix;
use synth_engine::SynthEngine;

use illuminator::IlluminationEngine;

use comms::BusStatus;

use rtt_target::{ rtt_init_print, rprintln };

#[entry]
fn main() -> ! {
    // rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let gclk0 = clocks.gclk0();
    let tc12 = &clocks.tc1_tc2(&gclk0).unwrap();

    let pins = bsp::Pins::new(peripherals.PORT);

    //Configure I2C
    // let sercom0_clock = &clocks.sercom0_core(&gclk0).unwrap();
    // let pads = i2c::Pads::new(pins.sda, pins.scl);

    // let mut sercom0 = peripherals.SERCOM0;

    // sercom0.enable_apb_clock(&peripherals.PM);

    // let output_pin = Some(pins.int.into_push_pull_output());

    // i2c_peripheral::configure_bus_status(output_pin);

    // i2c_peripheral::configure_sercom0(sercom0, 0x22);

    // unsafe {
    //     core.NVIC.set_priority(interrupt::SERCOM0, 1);
    //     NVIC::unmask(interrupt::SERCOM0);
    // }

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut synth_engine = SynthEngine::new();

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

    let mut led_timer = TimerCounter::tc1_(tc12, peripherals.TC1, &mut peripherals.PM);
    led_timer.start(MegaHertz::MHz(7).into_duration());

    let mut led_data_pin = pins.led_data.into_push_pull_output();
    led_data_pin.set_drive_strength(true);

    let mut led_strand = ws2812::Ws2812::new(led_timer, led_data_pin);

    let mut illuminator = IlluminationEngine::new(&mut led_strand);

    let delta_t_ms = 1;

    loop {
        // interrupt_helpers::free(|cs| unsafe {
        //     if let Some(comms_status) = i2c_peripheral::BUS_STATUS.borrow(cs).borrow_mut().as_mut() {
        //         if let Some(command) = comms_status.process() {

        //         }
        //     }
        // });

        let keystate = keyboard_matrix.scan(&mut delay);

        // Update Synth Engine state
        synth_engine.update(&keystate);

        illuminator.update(delta_t_ms, &keystate, &synth_engine.state);

        illuminator.render();
    }
}
