use crate::data::*;
use smart_leds::hsv::RGB8;

const NORMAL_STRIKE_COLOR: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
const NORMAL_SUSTAIN_COLOR_1: RGB8 = RGB8 { r: 0, g: 64, b: 0 };
const NORMAL_SUSTAIN_COLOR_2: RGB8 = RGB8 { r: 32, g: 32, b: 0 };

const OCTAVE_STRIKE_COLOR: RGB8 = RGB8 {
    r: 0,
    g: 64,
    b: 255,
};
const OCTAVE_SUSTAIN_COLOR_1: RGB8 = RGB8 { r: 0, g: 0, b: 32 };
const OCTAVE_SUSTAIN_COLOR_2: RGB8 = RGB8 { r: 0, g: 16, b: 48 };

const OCTAVE_SELECTED_COLOR_1: RGB8 = RGB8 { r: 0, g: 0, b: 32 };
const OCTAVE_SELECTED_COLOR_2: RGB8 = RGB8 { r: 0, g: 16, b: 48 };

const SUSTAIN_DURATION: u32 = 1000;
const FADE_DURATION: u32 = 1000;

const NEIGHBOR_COLORS: [RGB8; 3] = [
    RGB8 {
        r: 128,
        g: 255,
        b: 219,
    },
    RGB8 {
        r: 239,
        g: 64,
        b: 161,
    },
    RGB8 { r: 17, g: 64, b: 4 },
];

const NEIGHBOR_COLORS_SUSTAIN: [RGB8; 3] = [
    RGB8 { r: 128, g: 0, b: 0 },
    RGB8 { r: 0, g: 0, b: 0 },
    RGB8 { r: 0, g: 0, b: 0 },
];

const NEIGHBOR_COLORS_OCTAVE: [RGB8; 3] = [
    RGB8 { r: 128, g: 0, b: 0 },
    RGB8 { r: 0, g: 0, b: 0 },
    RGB8 { r: 0, g: 0, b: 0 },
];

fn keypress_compute(
    STRIKE_COLOR: RGB8,
    SUSTAIN_COLOR_1: RGB8,
    SUSTAIN_COLOR_2: RGB8,
    data: u32,
    duration: u32,
) -> RGB8 {
    if duration < SUSTAIN_DURATION {
        let percent = min(100, (duration / 10) as u8);

        let color = STRIKE_COLOR.fade(SUSTAIN_COLOR_1, percent);

        return color;
    } else {
        let duration = duration - SUSTAIN_DURATION;

        let percent = min(100, ((duration % SUSTAIN_DURATION) / 10) as u8);

        let color = if duration / 1000 % 2 == 0 {
            SUSTAIN_COLOR_1.fade(SUSTAIN_COLOR_2, percent)
        } else {
            SUSTAIN_COLOR_2.fade(SUSTAIN_COLOR_1, percent)
        };

        return color;
    }
}

pub struct NormalKeyPressAnimation {}

impl PixelAnimation for NormalKeyPressAnimation {
    fn compute(data: u32, duration: u32) -> RGB8 {
        keypress_compute(
            NORMAL_STRIKE_COLOR,
            NORMAL_SUSTAIN_COLOR_1,
            NORMAL_SUSTAIN_COLOR_2,
            data,
            duration,
        )
    }
}

pub struct OctaveKeyPressAnimation {}

impl PixelAnimation for OctaveKeyPressAnimation {
    fn compute(data: u32, duration: u32) -> RGB8 {
        keypress_compute(
            OCTAVE_STRIKE_COLOR,
            OCTAVE_SUSTAIN_COLOR_1,
            OCTAVE_SUSTAIN_COLOR_2,
            data,
            duration,
        )
    }
}

pub struct SelectedOctaveAnimation {}

impl PixelAnimation for SelectedOctaveAnimation {
    fn compute(data: u32, duration: u32) -> RGB8 {
        if duration < SUSTAIN_DURATION {
            let percent = min(100, (duration / 10) as u8);

            let previousColor = RGB8::deserialize(data);

            let color = previousColor.fade(OCTAVE_SELECTED_COLOR_1, percent);

            return color;
        } else {
            let duration = duration - SUSTAIN_DURATION;

            let percent = min(100, ((duration % SUSTAIN_DURATION) / 10) as u8);

            let color = if duration / 1000 % 2 == 0 {
                OCTAVE_SELECTED_COLOR_1.fade(OCTAVE_SELECTED_COLOR_2, percent)
            } else {
                OCTAVE_SELECTED_COLOR_2.fade(OCTAVE_SELECTED_COLOR_1, percent)
            };

            return color;
        }
    }
}

pub struct KeyFadeAnimation {}

impl KeyFadeAnimation {
    pub fn is_complete(duration: u32) -> bool {
        duration > FADE_DURATION
    }
}

impl PixelAnimation for KeyFadeAnimation {
    fn compute(data: u32, duration: u32) -> RGB8 {
        let percent = min(100, (duration / 10) as u8);
        let original_color = RGB8::deserialize(data);

        original_color.fade(RGB8::default(), percent)
    }
}

pub struct KeyRadiantAnimation {}

impl PixelAnimation for KeyRadiantAnimation {
    fn compute(data: u32, duration: u32) -> RGB8 {
        let radius = data;
        let percent = min(100, (duration / 10) as u8);
        let original_color = RGB8::deserialize(data);

        original_color.fade(RGB8::default(), percent)
    }
}

mod test {
    use crate::{data::*, keystrike_animation::FADE_DURATION};
    use smart_leds::hsv::RGB8;

    #[test]
    fn test_fade_animation_at_start() {
        let data: u32 = RGB8 { r: 255, g: 0, b: 0 }.serialize();
        let duration = 0;

        let result = super::KeyFadeAnimation::compute(data, duration);

        assert_eq!(result, RGB8 { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn test_fade_animation_halfway() {
        let data: u32 = RGB8 { r: 255, g: 0, b: 0 }.serialize();
        let duration = FADE_DURATION / 2;

        let result = super::KeyFadeAnimation::compute(data, duration);

        assert_eq!(result, RGB8 { r: 127, g: 0, b: 0 });
    }

    #[test]
    fn test_fade_animation_complete() {
        let data: u32 = RGB8 { r: 255, g: 0, b: 0 }.serialize();
        let duration = FADE_DURATION;

        let result = super::KeyFadeAnimation::compute(data, duration);

        assert_eq!(result, RGB8 { r: 0, g: 0, b: 0 });
    }
}
