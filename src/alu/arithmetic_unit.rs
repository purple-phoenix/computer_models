use crate::primitives::booleans::MBoolean;
use crate::logic_gates::logical_gates::*;
use crate::primitives::numbers::*;
use crate::primitives::byte::{MByte, make_empty_byte};

pub fn make_half_adder() -> fn(&MBoolean, &MBoolean) -> (MBoolean, MBoolean){
    |input_a, input_b| {
        let sum_bit = make_xor()(input_a, input_b);
        let carry_bit = make_and()(input_a, input_b);
        return (sum_bit, carry_bit);
    }
}

pub fn make_full_adder() -> fn(&MBoolean, &MBoolean, &MBoolean) -> (MBoolean, MBoolean) {
    |input_a, input_b, input_c| {
        let half_add_ab = make_half_adder()(input_a, input_b);
        let sum_ab = half_add_ab.0;
        let carry_ab = half_add_ab.1;
        let sum_carry_abc = make_half_adder()(&sum_ab, input_c);
        let total_sum = sum_carry_abc.0;
        let carry_abc = sum_carry_abc.1;
        let total_carry = make_or()(&carry_ab, &carry_abc);
        return (total_sum, total_carry)
    }

}

pub fn make_adder() -> fn(&Number, &Number) -> Number {
    return |x, y|
        return add(x, y,)

}

fn add(x: &dyn HasBytes, y: &dyn HasBytes) -> Number {
    let mut x_bytes = x.get_bytes();
    let mut y_bytes = y.get_bytes();
    let num_x_bytes = x_bytes.len();
    let num_y_bytes = y_bytes.len();
    if num_x_bytes > num_y_bytes {
        let filler_bytes_to_add = num_x_bytes - num_y_bytes;
        let mut new_x_bytes = vec![make_empty_byte(); filler_bytes_to_add];
        new_x_bytes.append(&mut x_bytes);
        return add_checked(new_x_bytes, y_bytes);
    }
    else if num_y_bytes > num_x_bytes {
        let filler_bytes_to_add = num_y_bytes - num_x_bytes;
        let mut new_y_bytes = vec![make_empty_byte(); filler_bytes_to_add];
        new_y_bytes.append(&mut y_bytes);
        return add_checked(x_bytes, new_y_bytes);
    }
    else {
        return add_checked(x_bytes, y_bytes);
    }


}

fn add_checked(x: Vec<MByte>, y: Vec<MByte>) -> Number {
    println!("\n\n\n\n\n\n\n");
    let num_bytes = x.len();
    let carry_bit = &mut MBoolean::FALSE;
    let mut byte_vector = vec![];
    for a_byte in 0..num_bytes {
        let byte_index = num_bytes - 1 - a_byte;
        println!("Adding byte index {:?}", byte_index);
        println!("Carry bit is {:?}", carry_bit);
        let mut x_byte = x[byte_index].clone();
        //Convert to LE
        x_byte.reverse();
        let mut y_byte = y[byte_index].clone();
        // Convert to LE
        y_byte.reverse();
        let (mut sum_byte, new_carry_bit) =
            eight_bit_carry_adder(&x_byte, &y_byte, &carry_bit.clone());
        //Convert sum byte to big endian for return
        sum_byte.reverse();
        byte_vector.push(sum_byte);

        *carry_bit = new_carry_bit;
    }

    //Convert byte vector to Big Endian
    byte_vector.reverse();

    if num_bytes <= 1 {
        return Number::Int8(Int8::new(byte_vector))
    }
    else if num_bytes <= 4 {
        return Number::Int32(Int32::new(byte_vector))
    }
    else {
        panic!("No number implemented which can store {} bytes");
    }
}

fn eight_bit_carry_adder_init(x: &MByte, y: &MByte) -> (MByte, MBoolean) {
    return eight_bit_carry_adder(x, y, &MBoolean::FALSE)
}

fn eight_bit_carry_adder(x: &MByte, y: &MByte, carry_bit: &MBoolean) -> (MByte, MBoolean) {
    let first_bit_x = &x[0];
    let first_bit_y = &y[0];
    let rest_x_bits = &x[1..];
    let rest_y_bits = &y[1..];
    let (sum0, carry0) = make_full_adder()(first_bit_x, first_bit_y, carry_bit);
    println!("Sum0 {:?}, Carry0 {:?}", sum0, carry0);
    let mut sum_byte = Vec::with_capacity(8);
    sum_byte.push(sum0);
    return eight_bit_carry_adder_helper(rest_x_bits,
                                        rest_y_bits,
                                        &mut sum_byte,
                                        &carry0);
}

fn eight_bit_carry_adder_helper(x: &[MBoolean],
                                y: &[MBoolean],
                                sum_byte: &mut MByte,
                                carry_bit: &MBoolean) -> (MByte, MBoolean) {
    if x.is_empty() {
        return (sum_byte.clone(), *carry_bit);
    }
    let next_x_bit = x[0];
    let next_y_bit = y[0];
    let rest_x_bits = &x[1..];
    let rest_y_bits = &y[1..];
    println!("Next X bit {:?}  next y bit {:?} passed carry bit   {:?}",
             next_x_bit, next_y_bit, carry_bit);
    let (next_sum, next_carry) =
        make_full_adder()(&next_x_bit, &next_y_bit, carry_bit);
    println!("Next sum {:?} next carry {:?}", next_sum, next_carry);
    sum_byte.push(next_sum);
    return eight_bit_carry_adder_helper(rest_x_bits,
                                        rest_y_bits,
                                        sum_byte,
                                        &next_carry);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::byte::make_byte_with_padding;

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
        let five = Int8::make_int8(&5);
        let ten = Int8::make_int8(&10);
        assert_eq!(ten.to_number(), make_adder()(&five.to_number(), &five.to_number()));

        let two_fifty_five = Int32::make_int32(&255);
        let one = Int32::make_int32(&1);
        let two_fifty_six = Int32::make_int32(&256);
        assert_eq!(make_adder()(&two_fifty_five.to_number(),&one.to_number()),
                   two_fifty_six.to_number())
    }


    #[test]
    fn test_eight_bit_carry_adder() {
        let mut five_byte =
            make_byte_with_padding(vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE]);
        //Convert to LE for 8 bit adder
        five_byte.reverse();
        let mut ten_byte =
            make_byte_with_padding(
                vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE, MBoolean::FALSE]);
        //Convert to LE for 8 bit adder
        ten_byte.reverse();

        assert_eq!(eight_bit_carry_adder_init(&five_byte, &five_byte),
                   (ten_byte, MBoolean::FALSE));

        let mut one_twenty_eight_byte = make_byte_with_padding(
            vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE,
                 MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE]);
        //Convert to LE for 8 bit adder
        one_twenty_eight_byte.reverse();
        let mut byte_after_overflow = make_byte_with_padding(vec![MBoolean::FALSE; 8]);
        //Convert to LE for 8 bit adder
        byte_after_overflow.reverse();
        assert_eq!(eight_bit_carry_adder_init(&one_twenty_eight_byte, &one_twenty_eight_byte),
                   (byte_after_overflow, MBoolean::TRUE))
    }
}