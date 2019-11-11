use crate::primitives::booleans::MBoolean;
use crate::logic_gates::logical_gates::{make_or, make_not, make_and};

struct GatedLatch {
    state: MBoolean
}

impl GatedLatch {
    pub fn update(&self, data_bit: MBoolean, write_bit: MBoolean) -> GatedLatch {
        let set_bit = make_and()(&data_bit, &write_bit);
        let not_data = make_not()(&data_bit);
        let reset_bit = make_and()(&not_data, &write_bit);
        let updated_state = make_and_or_latch()(self.state, set_bit, reset_bit);
        return GatedLatch::new(updated_state)
    }

    pub fn get_state(&self) -> MBoolean {
        return self.state;
    }
    pub fn new(new_state: MBoolean) -> GatedLatch {
        return GatedLatch{state: new_state}
    }
}

fn make_and_or_latch() -> fn(MBoolean, MBoolean, MBoolean) -> MBoolean {
    |previous_output, set, reset| {
        // Latch output is initially false before set
        let set_output_or = make_or()(&set, &previous_output);
        let not_reset = make_not()(&reset);
        let or_and_not_reset = make_and()(&set_output_or, &not_reset);
        return or_and_not_reset
    }
}


fn make_gated_latch() -> GatedLatch {
    return GatedLatch {state: MBoolean::FALSE}
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gated_latch() {
        // State of latch initialized to zero
        let gated_latch = make_gated_latch();

        assert_eq!(gated_latch.get_state(), MBoolean::FALSE);
        let gated_latch =
            gated_latch.update(MBoolean::FALSE, MBoolean::FALSE);
        assert_eq!(gated_latch.get_state(), MBoolean::FALSE);
        let gated_latch =
            gated_latch.update(MBoolean::TRUE, MBoolean::FALSE);
        assert_eq!(gated_latch.get_state(), MBoolean::FALSE);
        let gated_latch =
            gated_latch.update(MBoolean::FALSE, MBoolean::TRUE);
        assert_eq!(gated_latch.get_state(), MBoolean::FALSE);
        let gated_latch =
            gated_latch.update(MBoolean::TRUE, MBoolean::TRUE);
        assert_eq!(gated_latch.get_state(), MBoolean::TRUE);
        let gated_latch =
            gated_latch.update(MBoolean::TRUE, MBoolean::FALSE);
        assert_eq!(gated_latch.get_state(), MBoolean::TRUE);
        let gated_latch =
            gated_latch.update(MBoolean::FALSE, MBoolean::FALSE);
        assert_eq!(gated_latch.get_state(), MBoolean::TRUE);

    }

    #[test]
    fn test_and_or_latch() {
        let and_or_latch = make_and_or_latch();
        assert_eq!(and_or_latch(MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE),
                   MBoolean::FALSE);
        assert_eq!(and_or_latch(MBoolean::FALSE, MBoolean::TRUE, MBoolean::FALSE),
                   MBoolean::TRUE);
        assert_eq!(and_or_latch(MBoolean::TRUE, MBoolean::FALSE, MBoolean::FALSE),
                    MBoolean::TRUE);
        assert_eq!(and_or_latch(MBoolean::TRUE, MBoolean::TRUE, MBoolean::FALSE),
                   MBoolean::TRUE);

        // Always false on reset
        assert_eq!(and_or_latch(MBoolean::FALSE, MBoolean::FALSE, MBoolean::TRUE),
                   MBoolean::FALSE);
        assert_eq!(and_or_latch(MBoolean::FALSE, MBoolean::TRUE, MBoolean::TRUE),
                   MBoolean::FALSE);
        assert_eq!(and_or_latch(MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE),
                   MBoolean::FALSE);
        assert_eq!(and_or_latch(MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE),
                   MBoolean::FALSE);
    }
}