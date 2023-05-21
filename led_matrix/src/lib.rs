#![cfg_attr(not(test), no_std)]

use core::result::Result;
use core::result::Result::*;


pub enum PinState {
    TriState,
    High,
    Low
}

pub struct PinDefinition {
    pin_id: u32
}

pub trait MatrixPinDriver {
    fn set_mode(&self, ps: PinState);
}

pub struct LedMatrixDefinition {
    // pin_a: &'static T,
    // pin_b: &'static T,
    // pin_c: &'static T,
    // pin_d: &'static T,
    // pin_e: &'static T,
    pin_state: [u8; 20],
    driving_cycle: u8,
    driving_pin: MatrixPin,
}

#[derive(Debug)]
pub enum MatrixError {
    Err
}

pub trait LedMatrix {
    type Error: core::fmt::Debug;

    fn new(
        // matrix_a: &T,
        // matrix_b: &T,
        // matrix_c: &T,
        // matrix_d: &T,
        // matrix_e: &T,
    ) -> Result<LedMatrixDefinition, Self::Error>;

    fn set_value(&mut self, led: u8, value: u8);

    fn clear(&mut self);

    fn step(&mut self);
}

#[derive(PartialEq,Debug)]
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

const CYCLES: [Cycle; 4] = [
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

const LED_PIN_DRIVES: [(MatrixPin, MatrixPin); 20] = [
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

impl LedMatrix for LedMatrixDefinition//<T>
{
    fn new(
        // matrix_a: &'static T,
        // matrix_b: &'static T,
        // matrix_c: &'static T,
        // matrix_d: &'static T,
        // matrix_e: &'static T,
    ) -> Result<LedMatrixDefinition, Self::Error> {
        Ok(LedMatrixDefinition {
            // pin_a: matrix_a,
            // pin_b: matrix_b,
            // pin_c: matrix_c,
            // pin_d: matrix_d,
            // pin_e: matrix_e,

            pin_state: [0; 20],
            driving_cycle: 0,
            driving_pin: MatrixPin::PinA,
        })
    }

    fn set_value(&mut self, led: u8, value: u8) {
        if led >= led_pin_drives.len() as u8 {
            //TODO: Return error
            return;
        }

        self.pin_state[led as usize] = value;
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

                if self.driving_cycle == cycles.len() as u8 {
                    self.driving_cycle = 0;
                } 

                MatrixPin::PinA
            }
        }
    }

    type Error = MatrixError;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_begins_0th_cycle() {
        let under_test = LedMatrixDefinition::new().unwrap();

        assert_eq!(under_test.driving_cycle, 0);
    }

    #[test]
    fn matrix_begins_pin_a() {
        let under_test = LedMatrixDefinition::new().unwrap();

        assert_eq!(under_test.driving_pin, MatrixPin::PinA);
    }

    #[test]
    fn matrix_step_increments_pin() {
        let mut under_test = LedMatrixDefinition::new().unwrap();

        under_test.step();

        assert_eq!(under_test.driving_pin, MatrixPin::PinB);
    }

    #[test]
    fn matrix_5_steps_drives_a() {
        let mut under_test = LedMatrixDefinition::new().unwrap();

        for _ in 0..5 {
            under_test.step();
        }

        assert_eq!(under_test.driving_pin, MatrixPin::PinA);
    }

    #[test]
    fn matrix_5_steps_increments_cycle() {
        let mut under_test = LedMatrixDefinition::new().unwrap();

        for _ in 0..5 {
            under_test.step();
        }

        assert_eq!(under_test.driving_cycle, 1);
    }

    #[test]
    fn matrix_20_steps_cycle_is_0() {
        let mut under_test = LedMatrixDefinition::new().unwrap();

        for _ in 0..20 {
            under_test.step();
        }

        assert_eq!(under_test.driving_cycle, 0);
    }

    #[test]
    fn matrix_20_steps_driving_pin_is_a() {
        let mut under_test = LedMatrixDefinition::new().unwrap();

        for _ in 0..20 {
            under_test.step();
        }

        assert_eq!(under_test.driving_pin, MatrixPin::PinA);
    }
}