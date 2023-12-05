#![no_std]

use core::fmt::Error;

use keyboard_matrix::KeyboardState;
use smart_leds::{hsv::RGB8, SmartLedsWrite};

use rtt_target::{rtt_init_print, rprintln};

pub struct Illuminator<'a, StrandType> {
    led_strand: &'a mut StrandType,
    led_data: [RGB8; 21],
    needs_refresh: bool,
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

impl<'a, LedStrand> Illuminator<'a, LedStrand>
where
    LedStrand: SmartLedsWrite<Error = (), Color = RGB8>,
{
    pub fn new(led_strand: &'a mut LedStrand) -> Self {
        Self {
            led_strand: led_strand,
            led_data: [RGB8::default(); 21],
            needs_refresh: true,
        }
    }

    pub fn decay(&mut self) {
        for i in 0..21 {
            let mut pixel = &mut self.led_data[i];
        
            if pixel.r > 0 {
                pixel.r -= 1;
                self.needs_refresh = true;
            }
            if pixel.g > 0 {
                pixel.g -= 1;
                self.needs_refresh = true;
            }
            if pixel.b > 0 {
                pixel.b -= 1;
                self.needs_refresh = true;
            }
        }
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
        for i in 0..21 {
            if keyboard_state.pressed[i] {
                self.needs_refresh = true;

                self.led_data[i] = RGB8 { r: 0, g: 255, b: 0 };

                self.spread_to_adjacent(i as u8, RGB8 {
                    r: 128,
                    g: 0,
                    b: 128,
                }, 2);
            }
        }
    }

    fn spread_to_adjacent(&mut self, index: u8, color: RGB8, recurse_level: u8) {
        for i in 0..6 {
            let neighbor = ADJACENCY_BY_INDEX[index as usize][i];
            if neighbor != 255 {
                if self.led_data[neighbor as usize].r < color.r {
                    self.led_data[neighbor as usize].r = color.r;
                }
                if self.led_data[neighbor as usize].g < color.g {
                    self.led_data[neighbor as usize].g = color.g;
                }
                if self.led_data[neighbor as usize].b < color.b {
                    self.led_data[neighbor as usize].b = color.b;
                }

                if recurse_level > 0 {
                    self.spread_to_adjacent(neighbor, RGB8 {
                        r: color.r / 2,
                        g: color.g / 2,
                        b: color.b / 2,
                    }, recurse_level - 1);
                }
            }
        }
    }

    pub fn render(&mut self) {
        if self.needs_refresh {
            for i in 0..21 {
                rprintln!("{}: {} {} {}", i, self.led_data[i].r, self.led_data[i].g, self.led_data[i].b);
            }

            self.led_strand
                .write(self.led_data.iter().cloned())
                .unwrap();
            self.needs_refresh = false;
        }
    }
}
