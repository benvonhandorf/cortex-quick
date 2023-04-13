#![cfg_attr(not(test), no_std)]

use core::result::Result;

pub enum PinState {
    TriState,
    High,
    Low
}

pub trait MatrixPinDriver {
    fn set_mode(ps: PinState);
}

pub struct LedMatrixDefinition {
    pin_a: MatrixPinDriver,
    pin_b: MatrixPinDriver,
    pin_c: MatrixPinDriver,
    pin_d: MatrixPinDriver,
    pin_e: MatrixPinDriver,
    pin_state: [u8; 20],
    driving_cycle: u8,
    driving_pin: MatrixPin,
}
pub trait LedMatrix {
    type Error: core::fmt::Debug;

    fn new(
        matrix_a: impl MatrixPinDriver,
        matrix_b: impl MatrixPinDriver,
        matrix_c: impl MatrixPinDriver,
        matrix_d: impl MatrixPinDriver,
        matrix_e: impl MatrixPinDriver,
    ) -> Result<LedMatrixDefinition, Self::Error>;

    fn set_value(&mut self, led: u8, value: u8);

    fn clear(&mut self);

    fn step(&mut self);
}

enum MatrixPin {
    PinA,
    PinB,
    PinC,
    PinD,
    PinE,
}


struct Cycle {
    duration: u8, //Duration of this phase in ticks
    value: u8, //Value to check for
}

const cycles: [Cycle] = [
    Cycle {
        duration: 1,
        value: 10,
    },
    Cycle {
        duration: 2,
        value: 40,
    },
    Cycle {
        duration: 8,
        value: 128,
    },
    Cycle {
        duration: 16,
        value: 240,
    },
];

const led_pin_drives: [(MatrixPin, MatrixPin)] = [
    (MatrixPin::PinA, MatrixPin::PinB),
    (MatrixPin::PinA, MatrixPin::PinC),
    (MatrixPin::PinA, MatrixPin::PinD),
    (MatrixPin::PinA, MatrixPin::PinE),
    (MatrixPin::PinB, MatrixPin::PinA),
    (MatrixPin::PinB, MatrixPin::PinC),
    (MatrixPin::PinB, MatrixPin::PinD),
    (MatrixPin::PinB, MatrixPin::PinE),
    (MatrixPin::PinC, MatrixPin::PinA),
    (MatrixPin::PinC, MatrixPin::PinB),
    (MatrixPin::PinC, MatrixPin::PinD),
    (MatrixPin::PinC, MatrixPin::PinE),
    (MatrixPin::PinD, MatrixPin::PinA),
    (MatrixPin::PinD, MatrixPin::PinB),
    (MatrixPin::PinD, MatrixPin::PinC),
    (MatrixPin::PinD, MatrixPin::PinD),
    (MatrixPin::PinE, MatrixPin::PinA),
    (MatrixPin::PinE, MatrixPin::PinB),
    (MatrixPin::PinE, MatrixPin::PinC),
    (MatrixPin::PinE, MatrixPin::PinD),
];

impl LedMatrix for LedMatrixDefinition
{
    fn new(
        matrix_a: MatrixPin,
        matrix_b: MatrixPin,
        matrix_c: MatrixPin,
        matrix_d: MatrixPin,
        matrix_e: MatrixPin,
    ) -> Result<LedMatrixDefinition, Self::Error> {
        Ok(LedMatrixDefinition {
            pin_a: matrix_a,
            pin_b: matrix_b,
            pin_c: matrix_c,
            pin_d: matrix_d,
            pin_e: matrix_e,

            pin_state: [0; 20],
            driving_cycle: 0,
            driving_pin: MatrixPin::PinA,
        })
    }

    fn set_value(&mut self, led: u8, value: u8) {
        if led >= led_pin_drives.len() {
            panic!("LED {led} exceeds matrix definition length")
        }

        self.pin_state[led] = value;
    }

    fn clear(&mut self) {
        for value in self.pin_state.iter_mut() {
            *value = 0
        }
    }

    fn step(&mut self) {
        self.driving_pin = match self.driving_pin {
            MatrixPin::PinA => MatrixPin::PinB,
            MatrixPin::PinB => MatrixPin::PinC,
            MatrixPin::PinC => MatrixPin::PinD,
            MatrixPin::PinD => MatrixPin::PinE,
            MatrixPin::PinE => {
                self.driving_cycle += 1;

                if self.driving_cycle == cycles.len() {
                    self.driving_cycle = 0;
                } 

                MatrixPin::PinA
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        assert_eq!(2, 3);
    }
}