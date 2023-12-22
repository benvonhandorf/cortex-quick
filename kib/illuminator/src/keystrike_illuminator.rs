pub use crate::illuminator::Illuminator;

use crate::data::*;

use crate::keystrike_animation::*;

use keyboard_matrix::KeyboardState;
use synth_engine::SynthState;

use smart_leds::hsv::RGB8;

use rtt_target::rprintln;

#[derive(Clone, Copy, PartialEq)]
enum KeyType {
    Normal,
    Octave,
}

#[derive(Clone, Copy, PartialEq)]
enum KeyState {
    Off,
    Pressed,
    Selected,
    Fade,
    Radiant,
}

#[derive(Clone, Copy)]
struct KeyData {
    state: KeyState,
    data: u32,
    counter: u32,
}

impl KeyData {
    fn new() -> Self {
        Self {
            state: KeyState::Off,
            data: 0,
            counter: 0,
        }
    }
}

pub struct KeystrikeIlluminator {
    key_data: [KeyData; 21],
}

impl KeystrikeIlluminator {
    pub fn new() -> Self {
        Self {
            key_data: [KeyData::new(); 21],
        }
    }
}

impl KeystrikeIlluminator {
    fn keytype_for_index(key_index: usize) -> KeyType {
        match key_index {
            0..=7 => KeyType::Octave,
            _ => KeyType::Normal,
        }
    }

    fn compute_pixel_for_index(key_index: usize, key_data: &KeyData) -> Option<RGB8> {
        let key_type = KeystrikeIlluminator::keytype_for_index(key_index);
        KeystrikeIlluminator::compute_pixel(key_type, key_data)
    }

    fn compute_pixel(key_type: KeyType, key_data: &KeyData) -> Option<RGB8> {
        let color: Option<RGB8> = match key_data.state {
            KeyState::Pressed => match key_type {
                KeyType::Normal => Some(NormalKeyPressAnimation::compute(
                    key_data.data,
                    key_data.counter,
                )),
                KeyType::Octave => Some(OctaveKeyPressAnimation::compute(
                    key_data.data,
                    key_data.counter,
                )),
            },
            KeyState::Fade => Some(KeyFadeAnimation::compute(key_data.data, key_data.counter)),
            KeyState::Radiant => Some(KeyRadiantAnimation::compute(
                key_data.data,
                key_data.counter,
            )),
            KeyState::Selected => Some(SelectedOctaveAnimation::compute(
                key_data.data,
                key_data.counter,
            )),
            _ => None,
        };

        color
    }
}

impl Illuminator for KeystrikeIlluminator {
    fn update(
        &mut self,
        delta_t_ms: u32,
        keyboard_state: &KeyboardState,
        synth_state: &SynthState,
    ) {
        //Set selected octave
        self.key_data[synth_state.octave as usize - 1].state = KeyState::Selected;

        for key_index in 0..21 {
            let mut key_data = &mut self.key_data[key_index];

            match key_data.state {
                KeyState::Off => {
                    if keyboard_state.state[key_index] {
                        key_data.state = KeyState::Pressed;
                        key_data.counter = 0;

                        adjacency_recursion(
                            255,
                            key_index as u8,
                            1,
                            &mut |index, recurse_level| {
                                let neighbor_data = &mut self.key_data[index as usize];

                                if neighbor_data.state == KeyState::Off {
                                    neighbor_data.state = KeyState::Radiant;
                                    neighbor_data.data = recurse_level as u32;
                                    neighbor_data.counter = 0;
                                }
                            },
                        );
                    }
                }
                KeyState::Pressed => {
                    if !keyboard_state.state[key_index] {
                        let previous_color =
                            KeystrikeIlluminator::compute_pixel_for_index(key_index, key_data);

                        let previous_color = previous_color.unwrap_or(RGB8::default());

                        key_data.state = KeyState::Fade;
                        key_data.counter = 0;
                        key_data.data = previous_color.serialize();
                    } else {
                        key_data.counter += delta_t_ms;
                    }
                }
                KeyState::Fade => {
                    if keyboard_state.state[key_index] {
                        key_data.state = KeyState::Pressed;

                        adjacency_recursion(
                            255,
                            key_index as u8,
                            1,
                            &mut |index, recurse_level| {
                                let neighbor_data = &mut self.key_data[index as usize];

                                if neighbor_data.state == KeyState::Off {
                                    neighbor_data.state = KeyState::Radiant;
                                    neighbor_data.data = recurse_level as u32;
                                    neighbor_data.counter = 0;
                                }
                            },
                        );
                    } else {
                        key_data.counter += delta_t_ms;

                        if KeyFadeAnimation::is_complete(key_data.counter) {
                            key_data.state = KeyState::Off;
                            key_data.counter = 0;
                        }
                    }
                }
                KeyState::Radiant => {
                    if keyboard_state.state[key_index] {
                        key_data.state = KeyState::Pressed;

                        adjacency_recursion(
                            255,
                            key_index as u8,
                            1,
                            &mut |index, recurse_level| {
                                let neighbor_data = &mut self.key_data[index as usize];

                                if neighbor_data.state == KeyState::Off {
                                    neighbor_data.state = KeyState::Radiant;
                                    neighbor_data.data = recurse_level as u32;
                                    neighbor_data.counter = 0;
                                }
                            },
                        );
                    } else if key_data.counter > 50 {
                        let previous_color =
                            KeystrikeIlluminator::compute_pixel_for_index(key_index, key_data);

                        let previous_color = previous_color.unwrap_or(RGB8::default());

                        key_data.state = KeyState::Fade;
                        key_data.counter = 0;
                        key_data.data = previous_color.serialize();
                    } else {
                        key_data.counter += delta_t_ms;
                    }
                }
                KeyState::Selected => {
                    if synth_state.octave != (key_index as u8 + 1) {
                        //Fade previously selected octave
                        let previous_color =
                            KeystrikeIlluminator::compute_pixel_for_index(key_index, key_data);
                        let previous_color = previous_color.unwrap_or(RGB8::default());
                        key_data.state = KeyState::Fade;
                        key_data.counter = 0;
                        key_data.data = previous_color.serialize();
                    } else {
                        key_data.counter += delta_t_ms;
                    }
                }
            }
        }
    }

    fn render(&mut self, leds: &mut [RGB8; 21]) {
        let mut no_data = true;
        // rprintln!("R");

        for key_index in 0..21 {
            let key_data = &self.key_data[key_index];

            // rprintln!("K");

            let color = KeystrikeIlluminator::compute_pixel_for_index(key_index, key_data);

            if color.is_some() {
                leds[key_index] = color.unwrap();
                no_data = false;
            }
        }

        // if no_data {
        //     rprintln!("E");
        // }
    }
}

mod test {
    use crate::data::*;
    use crate::illuminator::Illuminator;
    use smart_leds::hsv::RGB8;

    use super::KeyState;

    impl core::fmt::Debug for KeyState {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                KeyState::Off => write!(f, "Off"),
                KeyState::Pressed => write!(f, "Pressed"),
                KeyState::Selected => write!(f, "Selected"),
                KeyState::Fade => write!(f, "Fade"),
                KeyState::Radiant => write!(f, "Radiant"),
            }
        }
    }

    #[test]
    fn test_no_update_has_no_state() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        for key_index in 0..21 {
            let key_data = illuminator.key_data[key_index];

            assert_eq!(
                key_data.state,
                super::KeyState::Off,
                "Key {} has state {:?}",
                key_index,
                key_data.state
            );
        }
    }

    #[test]
    fn test_no_state_with_octave_selects_octave() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        let mut keyboard_state = keyboard_matrix::KeyboardState::default();
        let mut synth_state = synth_engine::SynthState::new();

        synth_state.octave = 4;

        illuminator.update(0, &keyboard_state, &synth_state);

        assert_eq!(illuminator.key_data[3].state, super::KeyState::Selected);
        assert_eq!(illuminator.key_data[3].counter, 0);

        let mut leds = [RGB8::default(); 21];

        illuminator.render(&mut leds);

        assert_eq!(leds[3], crate::keystrike_animation::OCTAVE_SELECTED_COLOR_1);
    }

    #[test]
    fn test_with_keypress_18_shows_pressed() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        let mut keyboard_state = keyboard_matrix::KeyboardState::default();
        let mut synth_state = synth_engine::SynthState::new();

        synth_state.octave = 4;
        keyboard_state.state[18] = true;

        illuminator.update(0, &keyboard_state, &synth_state);

        illuminator.update(1000, &keyboard_state, &synth_state);

        for key_index in 8..21 {
            let key_data = illuminator.key_data[key_index];

            match key_index {
                18 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Pressed,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
                9 | 8 | 17 | 19 | 4 | 5 | 6 | 10 | 15 | 16 | 20 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Radiant,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
                _ => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Off,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
            }
        }
    }

    #[test]
    fn test_with_keypress_18_sustained_shows_pressed() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        let mut keyboard_state = keyboard_matrix::KeyboardState::default();
        let mut synth_state = synth_engine::SynthState::new();

        synth_state.octave = 4;
        keyboard_state.state[18] = true;

        illuminator.update(0, &keyboard_state, &synth_state);

        illuminator.update(60, &keyboard_state, &synth_state);

        illuminator.update(10, &keyboard_state, &synth_state);

        for key_index in 8..21 {
            let key_data = illuminator.key_data[key_index];

            match key_index {
                18 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Pressed,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
                9 | 8 | 17 | 19 | 4 | 5 | 6 | 10 | 15 | 16 | 20 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Fade,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
                _ => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Off,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
            }
        }
    }

    #[test]
    fn test_with_keypress_18_released_shows_fade() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        let mut keyboard_state = keyboard_matrix::KeyboardState::default();
        let mut synth_state = synth_engine::SynthState::new();

        synth_state.octave = 4;
        keyboard_state.state[18] = true;

        illuminator.update(0, &keyboard_state, &synth_state);

        keyboard_state.state[18] = false;

        illuminator.update(10, &keyboard_state, &synth_state);

        for key_index in 8..21 {
            let key_data = illuminator.key_data[key_index];

            match key_index {
                18 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Fade,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                    assert_eq!(
                        key_data.counter, 0,
                        "Key {} has counter {}",
                        key_index, key_data.counter
                    );
                }
                9 | 8 | 17 | 19 | 4 | 5 | 6 | 10 | 15 | 16 | 20 => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Radiant,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                    assert_eq!(
                        key_data.counter, 10,
                        "Key {} has counter {}",
                        key_index, key_data.counter
                    );
                }
                _ => {
                    assert_eq!(
                        key_data.state,
                        super::KeyState::Off,
                        "Key {} has state {:?}",
                        key_index,
                        key_data.state
                    );
                }
            }
        }
    }

    #[test]
    fn test_with_keypress_18_released_delay_shows_off() {
        let mut illuminator = super::KeystrikeIlluminator::new();

        let mut keyboard_state = keyboard_matrix::KeyboardState::default();
        let mut synth_state = synth_engine::SynthState::new();

        synth_state.octave = 4;
        keyboard_state.state[18] = true;

        illuminator.update(0, &keyboard_state, &synth_state);

        keyboard_state.state[18] = false;

        //First update to Fade sets counter to zero.  Also sets radiant keys to radiant expiration
        illuminator.update(50, &keyboard_state, &synth_state);
        // Expires radiant keys
        illuminator.update(10, &keyboard_state, &synth_state);
        illuminator.update(10, &keyboard_state, &synth_state);

        illuminator.update(500, &keyboard_state, &synth_state);

        illuminator.update(501, &keyboard_state, &synth_state);

        for key_index in 8..21 {
            let key_data = illuminator.key_data[key_index];

            assert_eq!(
                key_data.state,
                super::KeyState::Off,
                "Key {} has state {:?}",
                key_index,
                key_data.state
            );
        }
    }
}
