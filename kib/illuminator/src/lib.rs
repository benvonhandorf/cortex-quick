#![no_std]

use keyboard_matrix::KeyboardState;
use smart_leds::{hsv::RGB8, SmartLedsWrite};

// pub struct LedState {
//     pub leds: [RGB8; 21],
// }

// impl LedState {
//     pub fn new() -> Self {
//         Self {
//             leds: [RGB8::default(); 21],
//             brightness: 0,
//         }
//     }
// }

pub struct Illuminator<'a, LedStrand> {
    led_strand: &'a mut LedStrand,
    led_data: [RGB8; 21],
}

const ADJACENCY_BY_INDEX: [[u8; 6]; 21] = [
    [1, 12, 255, 255, 255, 255],
    [0, 2, 12, 11, 255, 255],
    [1, 3, 11, 255, 255, 255],
    [2, 4, 10, 255, 255, 255],
    [3, 5, 10, 9, 255, 255],
    [4, 6, 9, 8, 255, 255],
    [5, 7, 8, 255, 255, 255],
    [6, 255, 255, 255, 255, 255],
    [5, 6, 9, 18, 19, 255],
    [4, 5, 10, 8, 17, 18],
    [3, 4, 9, 16, 17, 255],
    [1, 2, 12, 14, 15, 255],
    [0, 1, 11, 13, 14, 255],
    [12, 14, 255, 255, 255, 255],
    [12, 11, 13, 15, 255, 255],
    [11, 14, 16, 255, 255, 255],
    [10, 15, 17, 255, 255, 255],
    [10, 9, 16, 18, 255, 255],
    [9, 8, 17, 19, 255, 255],
    [8, 18, 20, 255, 255, 255],
    [19, 255, 255, 255, 255, 255],
];

impl<LedStrand> Illuminator<'_, LedStrand>
where
    LedStrand: SmartLedsWrite<Error = ()>,
{
    pub fn new(led_strand: &mut LedStrand) -> Self {
        Self {
            led_strand: led_strand,
            led_data: [RGB8::default(); 21],
        }
    }

    pub fn decay(&mut self) {
        //TODO: Take time into account
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
        for i in 0..21 {
            if keyboard_state.state[i] {
                self.led_data[i] = RGB8 {
                    r: 255,
                    g: 255,
                    b: 255,
                };
            }
        }
    }

    pub fn render(&mut self) {
        self.led_strand
            .write(self.led_data.iter().cloned())
            .unwrap();
    }
}
