pub use crate::illuminator::Illuminator;

use crate::data::*;

use crate::keystrike_animation::*;

use keyboard_matrix::KeyboardState;
use synth_engine::SynthState;

use smart_leds::hsv::RGB8;

use rtt_target::rprintln;

pub struct PatternIlluminator {
    idle_time_ms: u32,
}

impl PatternIlluminator {
    pub fn new() -> Self {
        Self { idle_time_ms: 0 }
    }
}

impl Illuminator for PatternIlluminator {
    fn update(
        &mut self,
        delta_t_ms: u32,
        keyboard_state: &KeyboardState,
        synth_state: &SynthState,
    ) {
        if keyboard_state.depressed_count == 0 {
            self.idle_time_ms += delta_t_ms;
        } else {
            self.idle_time_ms = 0;
        }
    }

    fn render(&mut self, leds: &mut [RGB8; 21]) {
        if self.idle_time_ms > 10000 {
            for (index, pixel) in leds.iter_mut().enumerate() {
                let offset = ((index as u32 * 12) + (self.idle_time_ms % 5000)) % 255;

                *pixel = RGB8 {
                    r: offset as u8,
                    g: 0,
                    b: 0,
                };
            }
        }
    }
}
