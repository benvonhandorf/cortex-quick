use keyboard_matrix::KeyboardState;
use synth_engine::SynthState;
use smart_leds::hsv::RGB8;

pub trait Illuminator {
    fn update(&mut self, delta_t_ms: u32, keyboard_state: &KeyboardState, synth_engine: &SynthState);
    fn render(&mut self, leds: &mut [RGB8;21]);
}