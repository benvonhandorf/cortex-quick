use smart_leds::hsv::RGB8;

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

pub fn min(a: u8, b: u8) -> u8 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

    // pub fn decay(&mut self) {
    //     let mut modified = false;

    //     for i in 0..21 {
    //         let mut pixel = &mut self.led_data[i];

    //         if pixel.b > 0 {
    //             pixel.b -= min(3, pixel.b);
    //             modified = true;
    //             // continue;
    //         }

    //         if pixel.g > 0 {
    //             pixel.g -= min(2, pixel.g);
    //             modified = true;
    //             // continue;
    //         }
        
    //         if pixel.r > 0 {
    //             pixel.r -= 1;
    //             modified = true;
    //         }
    //     }

    //     self.needs_refresh = self.needs_refresh || modified;
    // }

pub trait PixelAnimation {
    fn compute(data: u32, duration: u32) -> RGB8;
}

pub trait PixelHelpers {
    fn serialize(&self) -> u32;
    fn deserialize(data: u32) -> RGB8;
    fn fade(&self, to: RGB8, percent: u8) -> RGB8;
    fn set_led_color(&mut self, index: u8, color: RGB8) -> bool;
}

impl PixelHelpers for RGB8 {
    fn serialize(&self) -> u32 {
        let mut result: u32 = 0;

        result |= self.r as u32;
        result |= (self.g as u32) << 8;
        result |= (self.b as u32) << 16;

        result
    }

    fn deserialize(data: u32) -> RGB8 {
        RGB8 {
            r: (data & 0xFF) as u8,
            g: ((data >> 8) & 0xFF) as u8,
            b: ((data >> 16) & 0xFF) as u8,
        }
    }

    fn fade(&self, to: RGB8, percent: u8) -> RGB8 {
        let second_percent = min(percent, 100) as u16;
        let first_percent = 100 - second_percent;

        let r = (((self.r as u16 * first_percent) + (to.r as u16 * second_percent)) / 100) as u8;
        let g = (((self.g as u16 * first_percent) + (to.g as u16 * second_percent)) / 100) as u8;
        let b = (((self.b as u16 * first_percent) + (to.b as u16 * second_percent)) / 100) as u8;

        RGB8 { r, g, b }
    }

    fn set_led_color(&mut self, index: u8, color: RGB8) -> bool {
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

        let mut modified = false;

        if self.r < color_mod.r {
            self.r = color_mod.r;
            modified = true;
        }
        if self.g < color_mod.g {
            self.g = color_mod.g;
            modified = true;
        }
        if self.b < color_mod.b {
            self.b = color_mod.b;
            modified = true;
        }

        modified
    }
}

pub fn adjacency_recursion<F>(previous_index: u8, index: u8, recurse_level: u8, callback: &F)
where F: Fn(u8, u8) {

    for i in 0..6 {
        let neighbor = ADJACENCY_BY_INDEX[index as usize][i];
        if neighbor != 255 && neighbor != previous_index {
            callback(neighbor, recurse_level);

            if recurse_level > 0 {
                adjacency_recursion(index, neighbor, recurse_level - 1, callback);
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let color = RGB8 { r: 255, g: 0, b: 0 };
        let data = color.serialize();
        let result = RGB8::deserialize(data);

        assert_eq!(result, color);

        let color = RGB8 { r: 0, g: 255, b: 0 };
        let data = color.serialize();
        let result = RGB8::deserialize(data);

        assert_eq!(result, color);

        let color = RGB8 { r: 0, g: 0, b: 255 };
        let data = color.serialize();
        let result = RGB8::deserialize(data);

        assert_eq!(result, color);

        let color = RGB8 { r: 100, g: 122, b: 140 };
        let data = color.serialize();
        let result = RGB8::deserialize(data);

        assert_eq!(result, color);
    }

    #[test]
    fn test_fade_color_at_start() {
        let color_start = RGB8 { r: 255, g: 0, b: 0 };
        let color_end = RGB8 { r: 0, g: 0, b: 0 };

        let result = color_start.fade(color_end, 0);

        assert_eq!(result, color_start);

    }
}