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

const DEBOUNCE_COUNTER: u8 = 100;

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

    #[test]
    fn test_new_state_does_not_change_when_debounce_is_gt_zero() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 5;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.state[0], false);
    }

    #[test]
    fn test_debounce_decrements_when_state_matches() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 5;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = false;

        let result = before_state.build_new(new_state);

        assert_eq!(result.debounce_counter[0], 4);
    }

    #[test]
    fn test_debounce_decrements_when_state_changes() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 5;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.debounce_counter[0], 4);
    }

    #[test]
    fn test_debounce_stays_zero_without_state_change() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = false;

        let result = before_state.build_new(new_state);

        assert_eq!(result.debounce_counter[0], 0);
    }

    #[test]
    fn test_pressed_counter_reflects_newly_pressed_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.pressed_count, 1);
    }

    #[test]
    fn test_pressed_reflects_newly_pressed_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.pressed[0], true);
    }

    #[test]
    fn test_pressed_omits_previously_pressed_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = true;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = true;

        let result = before_state.build_new(new_state);

        assert_eq!(result.pressed[0], false);
    }

    #[test]
    fn test_released_counter_reflects_newly_released_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = true;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = false;

        let result = before_state.build_new(new_state);

        assert_eq!(result.released_count, 1);
    }

    #[test]
    fn test_released_reflects_newly_released_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = true;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = false;

        let result = before_state.build_new(new_state);

        assert_eq!(result.released[0], true);
    }

    #[test]
    fn test_preleased_omits_previously_released_item() {
        let mut before_state = KeyboardState::default();
        before_state.state[0] = false;
        before_state.debounce_counter[0] = 0;

        let mut new_state: [bool; 21] = [false; 21];
        new_state[0] = false;

        let result = before_state.build_new(new_state);

        assert_eq!(result.released[0], false);
    }
}