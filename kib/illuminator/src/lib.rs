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

const STRIKE_COLOR : RGB8 = RGB8 { r: 0, g: 255, b: 0 };
const SUSTAIN_COLOR : RGB8  = RGB8 { r: 0, g: 64, b: 0 };
const NEIGHBOR_COLOR_BASE : RGB8 = RGB8 { r: 64, g: 0, b: 64 };

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
        let mut modified = false;

        for i in 0..21 {
            let mut pixel = &mut self.led_data[i];
        
            if pixel.r > 0 {
                pixel.r -= 1;
                modified = true;
            }
            if pixel.g > 0 {
                pixel.g -= 1;
                modified = true;
            }
            if pixel.b > 0 {
                pixel.b -= 1;
                modified = true;
            }
        }

        self.needs_refresh = self.needs_refresh || modified;
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
        let mut modified = false;

        for i in 0..21 {
            if keyboard_state.pressed[i] {
                modified = self.set_led_color(i as u8, STRIKE_COLOR) || modified;

                self.spread_to_adjacent(i as u8, NEIGHBOR_COLOR_BASE, 2);
            } else if keyboard_state.state[i] {
                modified = self.set_led_color(i as u8, SUSTAIN_COLOR) || modified;
            }
        }

        self.needs_refresh = self.needs_refresh || modified;
    }

    fn set_led_color(&mut self, index: u8, color: RGB8) -> bool{
        let color_mod = if index < 8 {
            //Reduce the brightness of the first row
                    RGB8 { 
                        r: color.r / 2,
                        g: color.g / 2,
                        b: color.b / 2,
                }
            } else {
                color
            };

        let mut pixel = &mut self.led_data[index as usize];
        let mut modified = false;

        if pixel.r < color_mod.r {
            pixel.r = color_mod.r;
            modified = true;
        }
        if pixel.g < color_mod.g {
            pixel.g = color_mod.g;
            modified = true;
        }
        if pixel.b < color_mod.b {
            pixel.b = color_mod.b;
            modified = true;
        }

        modified
    }

    fn spread_to_adjacent(&mut self, index: u8, color: RGB8, recurse_level: u8) {

        for i in 0..6 {
            let neighbor = ADJACENCY_BY_INDEX[index as usize][i];
            if neighbor != 255 {
                self.set_led_color(neighbor, color);

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

    fn print_led_data(&self) {
        for i in 0..21 {
            rprintln!("{}: {} {} {}", i, self.led_data[i].r, self.led_data[i].g, self.led_data[i].b);
        }
    }

    pub fn render(&mut self) {
        if self.needs_refresh {
            // self.print_led_data();

            self.led_strand
                .write(self.led_data.iter().cloned())
                .unwrap();
            self.needs_refresh = false;
        }
    }
}
