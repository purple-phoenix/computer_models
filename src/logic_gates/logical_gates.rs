use crate::primitives::booleans::{MBoolean};

fn make_not() -> fn(MBoolean) -> MBoolean {
    |input|
    match input {
         MBoolean::TRUE => MBoolean::FALSE,
         MBoolean::FALSE => MBoolean::TRUE
    }
}


fn make_and() -> fn(MBoolean, MBoolean) -> MBoolean {
    |inputA, inputB|{
        match (inputA, inputB) {
            (MBoolean::TRUE, MBoolean::TRUE) => MBoolean::TRUE,
            (MBoolean::TRUE, MBoolean::FALSE) => MBoolean::FALSE,
            (MBoolean::FALSE, MBoolean::FALSE) => MBoolean::FALSE,
            (MBoolean::FALSE, MBoolean::TRUE) => MBoolean::FALSE
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_not() {
        assert_eq!(MBoolean::TRUE, make_not()(MBoolean::FALSE));
        assert_eq!(MBoolean::FALSE, make_not()(MBoolean::TRUE));
    }

    #[test]
    fn test_make_and() {
        assert_eq!(MBoolean::TRUE, make_and()(MBoolean::TRUE, MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_and()(MBoolean::FALSE, MBoolean::TRUE));
        assert_eq!(MBoolean::FALSE, make_and()(MBoolean::FALSE, MBoolean::FALSE));
        assert_eq!(MBoolean::FALSE, make_and()(MBoolean::TRUE, MBoolean::FALSE));
    }

}