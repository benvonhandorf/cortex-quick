#![no_std]

use core::u8;

use keyboard_matrix::KeyboardState;

const MIDI_NOTE_OFFSET : u8 = 24; //0th note is C1
pub const NUM_NOTES : usize = 97; //8 octaves, 12 notes per octave, plus 1 extra C in octave 8

/// State of a note
#[derive(Clone, Copy, PartialEq)]
pub enum NoteState {
    Off,
    Pressed,
    Sustain,
    Release,
}

impl NoteState {
    pub fn to_int(&self) -> u8 {
        match self {
            NoteState::Off => 0,
            NoteState::Pressed => 1,
            NoteState::Sustain => 2,
            NoteState::Release => 3,
        }
    }
}

impl NoteState {
    pub fn is_active(&self) -> bool {
        matches!(self, NoteState::Pressed | NoteState::Sustain | NoteState::Release)
    }

    fn activate(&self) -> NoteState {
        match self {
            NoteState::Off => NoteState::Pressed,
            NoteState::Pressed => NoteState::Sustain,
            NoteState::Sustain => NoteState::Sustain,
            NoteState::Release => NoteState::Pressed,
        }
    }

    fn deactivate(&self) -> NoteState {
        match self {
            NoteState::Off => NoteState::Off,
            NoteState::Pressed => NoteState::Release,
            NoteState::Sustain => NoteState::Release,
            NoteState::Release => NoteState::Off,
        }
    }
}

pub struct SynthState { 
    pub octave: u8, // 1 - 8
    pub note_index_state: [NoteState; NUM_NOTES], // Tuning from C1 to C9 (extra C in octave 8).  Requires MIDI_NOTE_OFFSET to be accurate midi note value.
}


impl SynthState {
    pub fn new() -> Self {
        Self {
            octave: 4,
            note_index_state: [NoteState::Off; NUM_NOTES],
        }
    }
    
    fn index_to_note_offset(&self, idx: u8) -> u8 {
        match idx {
            11=>3, //D#
            12=>1, //C#
            13=>0, //C
            14=>2, //D
            10=>6, //F#
            15=>4, //E
            16=>5, //F
            17=>7, //G
            8=>10, //A#
            9=>8, //G#
            18=>9, //A
            19=>11, //B
            20=>12, //C2
            _=>0,
        }
    }

    pub fn note_offset_to_index(&self, note_offset: u8) -> u8 {
        match note_offset {
            3=>11, //D#
            1=>12, //C#
            0=>13, //C
            2=>14, //D
            6=>10, //F#
            4=>15, //E
            5=>16, //F
            7=>17, //G
            10=>8, //A#
            8=>9, //G#
            9=>18, //A
            11=>19, //B
            12=>20, //C2
            _=>0,
        }
    }

    fn octave_note_offset_to_note_index(octave: u8, note_offset: u8) -> u8 {
        let octave_offset = (octave - 1) * 12;

        octave_offset + note_offset
    }

    pub fn note_offset_to_note_index(&self, note_offset: u8) -> u8 {
        SynthState::octave_note_offset_to_note_index(self.octave, note_offset)
    }

    pub fn note_index_to_note_offset(&self, note_index: u8) -> u8 {
        let octave_offset = (self.octave - 1) * 12;
        
        note_index - octave_offset
    }

    fn index_to_note_index(&self, idx: u8) -> u8 {
        let note_offset = self.index_to_note_offset(idx);
        let note_index = self.note_offset_to_note_index(note_offset);

        note_index
    }

    pub fn note_index_to_midi(&self, note_index: u8) -> u8 {
        let octave_offset = (self.octave + 1) * 12;
        let midi_note = MIDI_NOTE_OFFSET + octave_offset + note_index;

        midi_note
    }

    fn activate_note_index(&mut self, note_index: u8) {
        let note_index = note_index as usize;
        self.note_index_state[note_index] = self.note_index_state[note_index].activate();
    }

    fn deactivate_note_index(&mut self, note_index: u8) {
        let note_index = note_index as usize;
        self.note_index_state[note_index] = self.note_index_state[note_index].deactivate();
    }
}

pub struct SynthEngine {
    pub state: SynthState,
}

impl SynthEngine {
    pub fn new() -> Self {
        Self {
            state: SynthState::new()
        }
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
        // Update Octave
        for i in 0..8 {
            if keyboard_state.pressed[i] {
                self.state.octave = i as u8 + 1;
            }
        }

        //Clear notes for non-current octaves
        for octave in 1..9 {
            if octave != self.state.octave {
                for i in 0..12 {
                    let note_index = SynthState::octave_note_offset_to_note_index(octave, i);
                    self.state.deactivate_note_index(note_index);
                }
            }
        }

        // Update Notes
        for i in 8..21 {
            let note_index = self.state.index_to_note_index(i);

            if keyboard_state.state[i as usize] {
                self.state.activate_note_index(note_index);
            } else {
                self.state.deactivate_note_index(note_index);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{SynthState, SynthEngine, MIDI_NOTE_OFFSET};

    #[test]
    fn octave_and_note_offset_for_C4_produce_expected_note_index() {
        let octave = 4;
        let note_index = SynthState::octave_note_offset_to_note_index(octave, 0);

        assert_eq!(note_index, 36);
        assert_eq!(note_index + MIDI_NOTE_OFFSET, 60);
    }

    #[test]
    fn octave_and_note_offset_for_C1_produce_note_index_0() {
        let octave = 1;
        let note_index = SynthState::octave_note_offset_to_note_index(octave, 0);

        assert_eq!(note_index, 0);
    }

    #[test]
    fn octave_and_note_offset_for_C9_produce_note_index_96() {
        let octave = 8;
        let note_index = SynthState::octave_note_offset_to_note_index(octave, 12);

        assert_eq!(note_index, 96);
    }

    #[test]
    fn note_offset_for_C4_produces_correct_note_offset() {
        let synth_state = SynthState::new();
        let note_offset = synth_state.note_index_to_note_offset(36);

        assert_eq!(note_offset, 0);
    }


    #[test]
    fn note_offset_for_C9_produces_correct_note_offset() {
        let mut synth_state = SynthState::new();
        synth_state.octave = 8;
        let note_offset = synth_state.note_index_to_note_offset(96);

        assert_eq!(note_offset, 12);
    }

    #[test]
    fn update_with_no_keys_pressed_works() {
        let mut synth_engine = SynthEngine::new();
        let keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.octave, 4);
    }

    #[test]
    fn update_with_no_key_pressed_shows_off() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Off.to_int());
    }

    #[test]
    fn update_with_no_key_pressed_shows_pressed() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        keyboard_state.state[13] = true;

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Pressed.to_int());
    }

    #[test]
    fn update_with_pressed_key_pressed_shows_sustain() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.state.note_index_state[36] = crate::NoteState::Pressed;

        keyboard_state.state[13] = true;

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Sustain.to_int());
    }

    #[test]
    fn update_with_sustained_key_pressed_shows_sustain() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.state.note_index_state[36] = crate::NoteState::Sustain;

        keyboard_state.state[13] = true;

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Sustain.to_int());
    }

    #[test]
    fn update_with_sustained_key_released_shows_release() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.state.note_index_state[36] = crate::NoteState::Sustain;

        keyboard_state.state[13] = false;

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Release.to_int());
    }

    #[test]
    fn update_with_released_key_released_shows_off() {
        let mut synth_engine = SynthEngine::new();
        let mut keyboard_state = keyboard_matrix::KeyboardState::default();

        synth_engine.state.note_index_state[36] = crate::NoteState::Release;

        keyboard_state.state[13] = false;

        synth_engine.update(&keyboard_state);

        assert_eq!(synth_engine.state.note_index_state[36].to_int(), crate::NoteState::Off.to_int());
    }

    #[test]
    fn nodestate_activate_pressed_is_sustain() {
        let under_test = crate::NoteState::Pressed;

        let result = under_test.activate();

        assert_eq!(result.to_int(), crate::NoteState::Sustain.to_int(), "Expected Pressed to activate to Sustain");
    }


    #[test]
    fn nodestate_none_pressed_is_pressed() {
        let under_test = crate::NoteState::Off;

        let result = under_test.activate();

        assert_eq!(result.to_int(), crate::NoteState::Pressed.to_int(), "Expected Off to activate to Pressed");
    }

    #[test]
    fn nodestate_activate_sustain_is_sustain() {
        let under_test = crate::NoteState::Sustain;

        let result = under_test.activate();

        assert_eq!(result.to_int(), crate::NoteState::Sustain.to_int(), "Expected Sustain to activate to Sustain");
    }

    #[test]
    fn nodestate_deactivate_sustain_is_release() {
        let under_test = crate::NoteState::Sustain;

        let result = under_test.deactivate();

        assert_eq!(result.to_int(), crate::NoteState::Release.to_int(), "Expected Sustain to deactivate to Release");
    }

    #[test]
    fn nodestate_deactivate_release_is_off() {
        let under_test = crate::NoteState::Release;

        let result = under_test.deactivate();

        assert_eq!(result.to_int(), crate::NoteState::Off.to_int(), "Expected Sustain to deactivate to Off");
    }
}