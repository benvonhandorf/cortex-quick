use keyboard_matrix::KeyboardState;
use smart_leds::hsv::RGB8;

pub struct LedState {
    pub leds: [RGB8; 21],
}

impl LedState {
    pub fn new() -> Self {
        Self {
            leds: [RGB8::default(); 21],
            brightness: 0,
        }
    }
}