#![no_std]

mod keyboard_state;

use crate::keyboard_state::KeyboardState;
use embedded_hal::digital::v2::{InputPin, OutputPin};

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

        keystate[0] = self.col_p.is_high().ok().unwrap();
        keystate[1] = self.col_o.is_high().ok().unwrap();
        keystate[2] = self.col_n.is_high().ok().unwrap();
        keystate[3] = self.col_m.is_high().ok().unwrap();

        self.row_a.set_low().ok();
        self.row_b.set_high().ok();

        keystate[4] = self.col_m.is_high().ok().unwrap();
        keystate[5] = self.col_n.is_high().ok().unwrap();
        keystate[6] = self.col_o.is_high().ok().unwrap();
        keystate[7] = self.col_p.is_high().ok().unwrap();

        self.row_b.set_low().ok();
        self.row_c.set_high().ok();

        keystate[11] = self.col_n.is_high().ok().unwrap();
        keystate[12] = self.col_o.is_high().ok().unwrap();
        keystate[13] = self.col_p.is_high().ok().unwrap();
        keystate[14] = self.col_m.is_high().ok().unwrap();

        self.row_c.set_low().ok();
        self.row_d.set_high().ok();

        keystate[10] = self.col_o.is_high().ok().unwrap();
        keystate[15] = self.col_m.is_high().ok().unwrap();
        keystate[16] = self.col_p.is_high().ok().unwrap();
        keystate[17] = self.col_n.is_high().ok().unwrap();

        self.row_d.set_low().ok();
        self.row_e.set_high().ok();

        keystate[8] = self.col_o.is_high().ok().unwrap();
        keystate[9] = self.col_m.is_high().ok().unwrap();
        keystate[18] = self.col_n.is_high().ok().unwrap();
        keystate[19] = self.col_p.is_high().ok().unwrap();
        keystate[20] = self.col_q.is_high().ok().unwrap();

        self.row_e.set_low().ok();

        self.keyboard_state = self.keyboard_state.build_new(keystate);

        return self.keyboard_state;
    }
}