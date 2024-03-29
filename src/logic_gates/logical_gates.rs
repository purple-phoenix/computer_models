use crate::primitives::booleans::{MBoolean};

pub fn make_not() -> fn(&MBoolean) -> MBoolean {
    |input|
    match input {
         MBoolean::TRUE => MBoolean::FALSE,
         MBoolean::FALSE => MBoolean::TRUE
    }
}


pub fn make_and() -> fn(&MBoolean, &MBoolean) -> MBoolean {
    |input_a, input_b|{
        match (input_a, input_b) {
            (MBoolean::TRUE, MBoolean::TRUE) => MBoolean::TRUE,
            (MBoolean::TRUE, MBoolean::FALSE) => MBoolean::FALSE,
            (MBoolean::FALSE, MBoolean::FALSE) => MBoolean::FALSE,
            (MBoolean::FALSE, MBoolean::TRUE) => MBoolean::FALSE
        }
    }
}

pub fn make_or() -> fn(&MBoolean, &MBoolean) -> MBoolean {
    |input_a, input_b|{
        match (input_a, input_b) {
            (MBoolean::TRUE, MBoolean::TRUE) => MBoolean::TRUE,
            (MBoolean::TRUE, MBoolean::FALSE) => MBoolean::TRUE,
            (MBoolean::FALSE, MBoolean::FALSE) => MBoolean::FALSE,
            (MBoolean::FALSE, MBoolean::TRUE) => MBoolean::TRUE
        }
    }
}

pub fn make_xor() -> fn(&MBoolean, &MBoolean) -> MBoolean {
    |input_a, input_b| {
        let or_output = make_or()(input_a, input_b);
        let and_output = make_and()(input_a, input_b);
        let notted_and = make_not()(&and_output);
        return make_and()(&or_output, &notted_and);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_not() {
        assert_eq!(MBoolean::TRUE, make_not()(&MBoolean::FALSE));
        assert_eq!(MBoolean::FALSE, make_not()(&MBoolean::TRUE));
    }

    #[test]
    fn test_make_and() {
        assert_eq!(MBoolean::TRUE, make_and()(&MBoolean::TRUE, &MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_and()(&MBoolean::FALSE, &MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_and()(&MBoolean::FALSE, &MBoolean::FALSE));
        assert_eq!(MBoolean::FALSE, make_and()(&MBoolean::TRUE, &MBoolean::FALSE));
    }

    #[test]
    fn test_make_or() {
        assert_eq!(MBoolean::TRUE, make_or()(&MBoolean::TRUE, &MBoolean::TRUE));
        assert_eq!(MBoolean::TRUE, make_or()(&MBoolean::FALSE, &MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_or()(&MBoolean::FALSE, &MBoolean::FALSE));
        assert_eq!(MBoolean::TRUE, make_or()(&MBoolean::TRUE, &MBoolean::FALSE));
    }

    #[test]
    fn test_make_xor() {
        assert_eq!(MBoolean::FALSE, make_xor()(&MBoolean::TRUE, &MBoolean::TRUE));
        assert_eq!(MBoolean::TRUE, make_xor()(&MBoolean::FALSE, &MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_xor()(&MBoolean::FALSE, &MBoolean::FALSE));
        assert_eq!(MBoolean::TRUE, make_xor()(&MBoolean::TRUE, &MBoolean::FALSE));
    }

}