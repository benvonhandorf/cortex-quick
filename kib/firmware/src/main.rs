#![no_std]
#![no_main]

mod kib_board;
// mod comms;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use kib_board as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use pac::interrupt;
use pac::{CorePeripherals, Peripherals};

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::*;
use hal::timer::*;

// use rtt_target::rprint;
use ws2812_timer_delay as ws2812;

use rtt_target::rtt_init_print;

use keyboard_matrix::KeyboardMatrix;
use synth_engine::SynthEngine;

use illuminator::IlluminationEngine;

static mut output_pin: Option<
    hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<hal::gpio::PushPull>>,
> = None;

use hal::sercom::i2c;

#[interrupt]
fn SERCOM0() {
    unsafe {
        output_pin.as_mut().unwrap().toggle().ok();

        // let i2cs = SERCOM0::ptr()
        //     .as_ref()
        //     .unwrap()
        //     .i2cs();

        // i2cs.intflag.write(|w| w.error().clear_bit());
    }
}

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

    //Configure I2C
    let sercom0_clock = &clocks.sercom0_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(pins.sda, pins.scl);
    let mut i2c = i2c::Config::new(
        &peripherals.PM,
        peripherals.SERCOM0,
        pads,
        sercom0_clock.freq(),
    )
    .baud(100.kHz())
    .enable();

    unsafe {
        output_pin = Some(pins.int.into_push_pull_output());
    }

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

    let mut led_data_pin = pins.led_data.into_push_pull_output();
    led_data_pin.set_drive_strength(true);

    let mut led_strand = ws2812::Ws2812::new(led_timer, led_data_pin);

    let mut illuminator = IlluminationEngine::new(&mut led_strand);

    let delta_t_ms = 3;

    loop {
        let keystate = keyboard_matrix.scan(&mut delay);

        // Update Synth Engine state
        synth_engine.update(&keystate);

        // print_synthengine(&synth_engine);

        // print_keystate(&keystate);

        illuminator.update(delta_t_ms, &keystate, &synth_engine.state);

        illuminator.render();

        if synth_engine.state.dirty {
            let mut data = [0u8; 4];
            let mut i = 0;

            for note_index in 0..synth_engine::NUM_NOTES {
                if synth_engine.state.note_index_state[i] == synth_engine::NoteState::Pressed {
                    data[i + 1] = synth_engine.state.note_index_to_midi(note_index as u8);
                } else if synth_engine.state.note_index_state[i] == synth_engine::NoteState::Release {
                    data[i + 1] = synth_engine.state.note_index_to_midi(note_index as u8) | 0x80;
                }
            }

            if i2c.write(0x20, &data).is_err() {
                unsafe {
                    output_pin.as_mut().unwrap().toggle().ok();
                }
            }
        }
    }
}

// fn print_keystate(keystate: &keyboard_matrix::KeyboardState) {
//     if keystate.pressed_count > 0 || keystate.released_count > 0 {
//         rprint!("Keys {}: ", keystate.depressed_count);
//         for i in 0..21 {
//             if keystate.state[i] {
//                 rprint!("{} ", i);
//             }
//         }

//         rprintln!("");
//     }
// }

// fn print_synthengine(synth_engine: &synth_engine::SynthEngine) {
// rprintln!("Octave: {}", synth_engine.state.octave);

// for note_index in 0..synth_engine::NUM_NOTES {
//     if synth_engine.state.note_index_state[note_index] != synth_engine::NoteState::Off {
//         rprintln!("Note: {} {}", synth_engine.state.note_index_to_midi(note_index as u8), synth_engine.state.note_index_state[note_index].to_int());
//     }
// }
// }
