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

                        adjacency_recursion(255, key_index as u8, 2, &|index, recurse_level| {
                            let mut neighbor_data = self.key_data[index as usize];

                            if neighbor_data.state == KeyState::Off {
                                neighbor_data.state = KeyState::Radiant;
                                neighbor_data.data = recurse_level as u32;
                                neighbor_data.counter = 0;
                            }
                        });
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
                    }
                }
                KeyState::Fade => {
                    if keyboard_state.state[key_index] {
                        key_data.state = KeyState::Pressed;
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
                    } else {
                        key_data.counter += delta_t_ms;
                    }
                }
                KeyState::Selected => {
                    if !synth_state.octave != key_index as u8 + 1 {
                        //Fade previously selected octave
                        let previous_color =
                            KeystrikeIlluminator::compute_pixel_for_index(key_index, key_data);
                        let previous_color = previous_color.unwrap_or(RGB8::default());
                        key_data.state = KeyState::Fade;
                        key_data.counter = 0;
                        key_data.data = previous_color.serialize();
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
