use atsamd_hal::ehal::digital::v2::{InputPin, OutputPin};

use rtt_target::{rtt_init_print, rprintln};

#[derive(Clone, Copy, Debug)]
pub struct KeyboardState {
    pub state: [bool; 21],
    pub debounce_counter: [u8; 21],
    pub pressed: [bool; 21],
    pub released: [bool; 21],
    pub depressed_count: u8,
    pub pressed_count: u8,
    pub released_count: u8,
}

const DEBOUNCE_COUNTER: u8 = 20;

impl KeyboardState {
    pub fn default() -> Self {
        Self {
            state: [false; 21],
            debounce_counter: [0; 21],
            pressed: [false; 21],
            released: [false; 21],
            depressed_count: 0,
            pressed_count: 0,
            released_count: 0,
        }
    }

    pub fn build_new(&self, new_state: [bool; 21]) -> Self {
        let mut debounced_state: [bool; 21] = [false; 21];
        let mut debounce_counter: [u8; 21] = self.debounce_counter;
        let mut pressed: [bool; 21] = [false; 21];
        let mut released: [bool; 21] = [false; 21];
        let mut depressed_count = 0;
        let mut pressed_count = 0;
        let mut released_count = 0;

        for i in 0..21 {
            if new_state[i] != self.state[i] {
                if debounce_counter[i] == 0 {
                    debounced_state[i] = new_state[i];
                    debounce_counter[i] = DEBOUNCE_COUNTER;
                } else {
                    debounced_state[i] = self.state[i];
                }
            } else {
                debounced_state[i] = self.state[i];
            }

            if debounce_counter[i] > 0 {
                debounce_counter[i] -= 1;
            }
            
            if debounced_state[i] && !self.state[i] {
                pressed[i] = true;
                pressed_count += 1;
            }
            if !debounced_state[i] && self.state[i] {
                released[i] = true;
                released_count += 1;
            }
            if debounced_state[i] {
                depressed_count += 1;
            }
        }

        Self {
            state: debounced_state,
            debounce_counter: debounce_counter,
            pressed: pressed,
            released: released,

            depressed_count: depressed_count,
            pressed_count: pressed_count,
            released_count: released_count,
        }
    }
}

pub struct KeyboardMatrix<ROWA, ROWB, ROWC, ROWD, ROWE, COLM, COLN, COLO, COLP, COLQ> {
    row_a: ROWA,
    row_b: ROWB,
    row_c: ROWC,
    row_d: ROWD,
    row_e: ROWE,

    col_n: COLN,
    col_m: COLM,
    col_o: COLO,
    col_p: COLP,
    col_q: COLQ,

    keyboard_state: KeyboardState,
}

#[derive(Debug, Clone)]
pub struct SlowError;

impl core::fmt::Display for SlowError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "SlowError")
    }
}

impl<ROWA, ROWB, ROWC, ROWD, ROWE, COLM, COLN, COLO, COLP, COLQ>
    KeyboardMatrix<ROWA, ROWB, ROWC, ROWD, ROWE, COLM, COLN, COLO, COLP, COLQ>
where
    ROWA: OutputPin,
    ROWB: OutputPin,
    ROWC: OutputPin,
    ROWD: OutputPin,
    ROWE: OutputPin,
    COLM: InputPin,
    COLN: InputPin,
    COLO: InputPin,
    COLP: InputPin,
    COLQ: InputPin,
{
    pub fn new(
        row_a: ROWA,
        row_b: ROWB,
        row_c: ROWC,
        row_d: ROWD,
        row_e: ROWE,

        col_n: COLN,
        col_m: COLM,
        col_o: COLO,
        col_p: COLP,
        col_q: COLQ,
    ) -> Self {
        Self {
            row_a,
            row_b,
            row_c,
            row_d,
            row_e,

            col_n,
            col_m,
            col_o,
            col_p,
            col_q,

            keyboard_state: KeyboardState::default(),
        }
    }

    pub fn scan(&mut self) -> KeyboardState {
        let mut keystate: [bool; 21] = [false; 21];

        self.row_a.set_high().ok();

        keystate[2] = self.col_n.is_high().ok().unwrap();
        keystate[3] = self.col_m.is_high().ok().unwrap();
        keystate[1] = self.col_o.is_high().ok().unwrap();
        keystate[0] = self.col_p.is_high().ok().unwrap();

        self.row_a.set_low().ok();
        self.row_b.set_high().ok();

        keystate[5] = self.col_n.is_high().ok().unwrap();
        keystate[4] = self.col_m.is_high().ok().unwrap();
        keystate[6] = self.col_o.is_high().ok().unwrap();
        keystate[7] = self.col_p.is_high().ok().unwrap();

        self.row_b.set_low().ok();
        self.row_c.set_high().ok();

        keystate[11] = self.col_n.is_high().ok().unwrap();
        keystate[10] = self.col_m.is_high().ok().unwrap();
        keystate[9] = self.col_o.is_high().ok().unwrap();
        keystate[8] = self.col_p.is_high().ok().unwrap();

        self.row_c.set_low().ok();
        self.row_d.set_high().ok();

        keystate[15] = self.col_n.is_high().ok().unwrap();
        keystate[12] = self.col_m.is_high().ok().unwrap();
        keystate[14] = self.col_o.is_high().ok().unwrap();
        keystate[13] = self.col_p.is_high().ok().unwrap();

        self.row_d.set_low().ok();
        self.row_e.set_high().ok();

        keystate[17] = self.col_n.is_high().ok().unwrap();
        keystate[16] = self.col_m.is_high().ok().unwrap();
        keystate[18] = self.col_o.is_high().ok().unwrap();
        keystate[19] = self.col_p.is_high().ok().unwrap();
        keystate[20] = self.col_q.is_high().ok().unwrap();

        self.row_e.set_low().ok();

        self.keyboard_state = self.keyboard_state.build_new(keystate);

        return self.keyboard_state;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use more_asserts::*;

    #[test]
    fn test_new_state_reflects_change_when_debounce_is_zero() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.state[0], true);
    }

    #[test]
    fn test_state_change_sets_debounce() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_gt!(result.debounce_counter[0], 0);
    }
}