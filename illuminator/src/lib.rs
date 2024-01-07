#![no_std]

mod illuminator;
mod data;
mod keystrike_illuminator;
mod keystrike_animation;
mod pattern_illuminator;

use illuminator::Illuminator;
use keystrike_illuminator::KeystrikeIlluminator;

use pattern_illuminator::PatternIlluminator;

use keyboard_matrix::KeyboardState;
use synth_engine::SynthState;

use smart_leds::{hsv::RGB8, SmartLedsWrite};

pub struct IlluminationEngine<'a, StrandType> {
    led_strand: &'a mut StrandType,
    led_data: [RGB8; 21],
    keystrike_illuminator: KeystrikeIlluminator,
    pattern_illuminator: PatternIlluminator,
}

impl<'a, LedStrand> IlluminationEngine<'a, LedStrand>
where
    LedStrand: SmartLedsWrite<Error = (), Color = RGB8>,
{
    pub fn new(led_strand: &'a mut LedStrand) -> Self {
        Self {
            led_strand: led_strand,
            led_data: [RGB8::default(); 21],
            keystrike_illuminator: KeystrikeIlluminator::new(),
            pattern_illuminator: PatternIlluminator::new(),
        }
    }

    pub fn update(&mut self, delta_t_ms: u32, keyboard_state: &KeyboardState, synth_state: &SynthState) {
        self.keystrike_illuminator.update(delta_t_ms, keyboard_state, synth_state);

        self.pattern_illuminator.update(delta_t_ms, keyboard_state, synth_state);
    }

    pub fn render(&mut self) {

        for i in 0..21 {
            self.led_data[i] = RGB8::default();
        }
        
        self.keystrike_illuminator.render(&mut self.led_data);

        self.pattern_illuminator.render(&mut self.led_data);

        self.led_strand
            .write(self.led_data.iter().cloned())
            .unwrap();
    }
}
