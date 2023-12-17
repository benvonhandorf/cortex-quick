#![no_std]

mod illuminator;
mod data;
mod keystrike_illuminator;
mod keystrike_animation;

use illuminator::Illuminator;
use keystrike_illuminator::KeystrikeIlluminator;

use keyboard_matrix::KeyboardState;
use synth_engine::SynthState;

use smart_leds::{hsv::RGB8, SmartLedsWrite};

pub struct IlluminationEngine<'a, StrandType> {
    led_strand: &'a mut StrandType,
    led_data: [RGB8; 21],
    // needs_refresh: bool,
    // skipped_update_count: u16,
    illuminator: KeystrikeIlluminator,
}

impl<'a, LedStrand> IlluminationEngine<'a, LedStrand>
where
    LedStrand: SmartLedsWrite<Error = (), Color = RGB8>,
{
    pub fn new(led_strand: &'a mut LedStrand) -> Self {
        Self {
            led_strand: led_strand,
            led_data: [RGB8::default(); 21],
            // needs_refresh: true,
            // skipped_update_count: 0,
            illuminator: KeystrikeIlluminator::new(),
        }
    }

    pub fn update(&mut self, delta_t_ms: u32, keyboard_state: &KeyboardState, synth_state: &SynthState) {
        self.illuminator.update(delta_t_ms, keyboard_state, synth_state);
    }

    // fn print_led_data(&self) {
    //     for i in 0..21 {
    //         rprintln!("{}: {} {} {}", i, self.led_data[i].r, self.led_data[i].g, self.led_data[i].b);
    //     }
    // }

    pub fn render(&mut self) {

        for i in 0..21 {
            self.led_data[i] = RGB8::default();
        }
        
        self.illuminator.render(&mut self.led_data);
            // self.print_led_data();

        self.led_strand
            .write(self.led_data.iter().cloned())
            .unwrap();
        // self.needs_refresh = false;

        // self.skipped_update_count = 0;
        // } else {
        //     self.skipped_update_count += 1;
        // }
    }
}
