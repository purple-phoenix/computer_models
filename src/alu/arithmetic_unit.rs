use crate::primitives::booleans::MBoolean;
use crate::logic_gates::logical_gates::*;

pub fn make_half_adder() -> fn(&MBoolean, &MBoolean) -> (MBoolean, MBoolean){
    |inputA, inputB| {
        let sum_bit = make_xor()(inputA, inputB);
        let carry_bit = make_and()(inputA, inputB);
        return (sum_bit, carry_bit);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_half_adder() {
        //0 + 0 = 0 no carry
        assert_eq!((MBoolean::FALSE, MBoolean::FALSE),
                make_half_adder()(&MBoolean::FALSE, &MBoolean::FALSE));
        //0 + 1 = 1 no carry
        assert_eq!((MBoolean::TRUE, MBoolean::FALSE),
                make_half_adder()(&MBoolean::FALSE, &MBoolean::TRUE));
        //1 + 0 = 1 no carry
        assert_eq!((MBoolean::TRUE, MBoolean::FALSE),
                make_half_adder()(&MBoolean::TRUE, &MBoolean::FALSE));
        //1 + 1 = 0 and carry
        assert_eq!((MBoolean::FALSE, MBoolean::TRUE),
                make_half_adder()(&MBoolean::TRUE, &MBoolean::TRUE));
    }
}