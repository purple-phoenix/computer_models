use crate::primitives::booleans::MBoolean;
use crate::logic_gates::logical_gates::*;
use crate::primitives::numbers::*;

pub fn make_half_adder() -> fn(&MBoolean, &MBoolean) -> (MBoolean, MBoolean){
    |inputA, inputB| {
        let sum_bit = make_xor()(inputA, inputB);
        let carry_bit = make_and()(inputA, inputB);
        return (sum_bit, carry_bit);
    }
}

pub fn make_full_adder() -> fn(&MBoolean, &MBoolean, &MBoolean) -> (MBoolean, MBoolean) {
    |inputA, inputB, inputC| {
        let half_add_ab = make_half_adder()(inputA, inputB);
        let sum_ab = half_add_ab.0;
        let carry_ab = half_add_ab.1;
        let sum_carry_abc = make_half_adder()(&sum_ab, inputC);
        let total_sum = sum_carry_abc.0;
        let carry_abc = sum_carry_abc.1;
        let total_carry = make_or()(&carry_ab, &carry_abc);
        return (total_sum, total_carry)
    }

}

pub fn make_adder(num: Number) -> Number {
    match num {
        int8 => make_adder_x_bit(8),
        i32 => make_adder_x_bit(32)
    }
}

fn make_adder_x_bit(num_bits: i32) -> Number {

    //STUB TODO
    return Number::int32(int32::make_zero())
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

    #[test]
    fn test_full_adder() {
        //0 + 0 + 0 = 0 no carry
        assert_eq!((MBoolean::FALSE, MBoolean::FALSE),
            make_full_adder()(&MBoolean::FALSE, &MBoolean::FALSE, &MBoolean::FALSE)
        );

        //0 + 0 + 1 = 1 no carry
        assert_eq!((MBoolean::TRUE, MBoolean::FALSE),
                   make_full_adder()(&MBoolean::FALSE, &MBoolean::FALSE, &MBoolean::TRUE));

        //0 + 1 + 0 = 1 no carry
        assert_eq!((MBoolean::TRUE, MBoolean::FALSE),
                   make_full_adder()(&MBoolean::FALSE, &MBoolean::TRUE, &MBoolean::FALSE));

        //1 + 0 + 0 = 1 no carry
        assert_eq!((MBoolean::TRUE, MBoolean::FALSE),
                   make_full_adder()(&MBoolean::TRUE, &MBoolean::FALSE, &MBoolean::FALSE));

        //0 + 1 + 1 = 0 and carry
        assert_eq!((MBoolean::FALSE, MBoolean::TRUE),
                   make_full_adder()(&MBoolean::FALSE, &MBoolean::TRUE, &MBoolean::TRUE));

        //1 + 0 + 1 = 0 and carry
        assert_eq!((MBoolean::FALSE, MBoolean::TRUE),
                   make_full_adder()(&MBoolean::TRUE, &MBoolean::FALSE, &MBoolean::TRUE));

        //1 + 1 + 0 = 0 and carry
        assert_eq!((MBoolean::FALSE, MBoolean::TRUE),
                   make_full_adder()(&MBoolean::TRUE, &MBoolean::TRUE, &MBoolean::FALSE));

        //1 + 1 + 1 = 1 and carry
        assert_eq!((MBoolean::TRUE, MBoolean::TRUE),
                   make_full_adder()(&MBoolean::TRUE, &MBoolean::TRUE, &MBoolean::TRUE));
    }

    #[test]
    fn test_adder() {

    }
}