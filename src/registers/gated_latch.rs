use crate::primitives::booleans::MBoolean;
use crate::logic_gates::logical_gates::{make_or, make_not, make_and};

struct GatedLatch {
    state: MBoolean
}

impl GatedLatch {
    pub fn update(&self, data_bit: MBoolean, write_bit: MBoolean) {

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
        /*
        assert_eq!(gated_latch(MBoolean::FALSE, MBoolean::FALSE), MBoolean::FALSE);
        assert_eq!(gated_latch(MBoolean::TRUE, MBoolean::FALSE), MBoolean::FALSE);
        assert_eq!(gated_latch(MBoolean::FALSE, MBoolean::TRUE), MBoolean::FALSE);
        assert_eq!(gated_latch(MBoolean::TRUE, MBoolean::TRUE), MBoolean::TRUE);
        assert_eq!(gated_latch(MBoolean::TRUE, MBoolean::FALSE), MBoolean::TRUE);
        assert_eq!(gated_latch(MBoolean::FALSE, MBoolean::FALSE), MBoolean::TRUE);
        */
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