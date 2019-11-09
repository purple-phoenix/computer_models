use crate::primitives::byte::{MByte, make_empty_byte, make_byte_with_padding};
use std::i32::*;
use crate::primitives::booleans::MBoolean;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Number {
    int8(int8),
    int32(int32)
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct int32 {
    bytes: Vec<MByte>
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct int8 {
    bytes: Vec<MByte>
}


impl int32 {

    pub fn make_zero() -> int32 {
        return int32 {bytes: vec![make_empty_byte(); 4]}
    }

    pub fn is_valid(&self) -> bool {
        self.bytes.len() == 4
    }

    pub fn make_int32(input: &usize) -> int32 {
        let num = make_intn(input, &32);
        match num {
            Number::int32(an_int32) => an_int32,
            _ => panic!("make_intn({}, 32) did not return an int32")
        }
    }

    pub fn to_number(&self) -> Number {
        return Number::int32(self.clone())
    }

}

impl int8 {

    pub fn make_zero() -> int8 {
        return int8 {bytes: vec![make_empty_byte()]}
    }

    pub fn is_valid(&self) -> bool {
        self.bytes.len() == 1
    }

    pub fn make_int8(input: &usize) -> int8 {
        let num = make_intn(input, &8);
        match num {
            Number::int8(an_int8) =>  an_int8,
            _ => {panic!("make_intn({}, 8) did not return an int8")}
        }
    }
    pub fn to_number(&self) -> Number {
        return Number::int8(self.clone())
    }
}

fn make_intn(input: &usize, num_bits: &usize) -> Number {
    let input_bytes = input.to_be_bytes();
    let byte_len = 8;

    let num_bytes = num_bits /byte_len;
    println!("{} has {} bytes", input, num_bytes);
    let mut byte_vector = Vec::with_capacity(num_bits /8);

    let mut byte_index = input_bytes.len() - num_bytes;
    for byte_num in 0..num_bytes {
        let mut bit_vector: Vec<MBoolean> = Vec::with_capacity(byte_len);

        let comparison_byte = input_bytes[byte_index];
        byte_index += 1;
        println!("Attempting to copy byte {:?}", comparison_byte);
        for n in 0..byte_len {
            let bit_n = comparison_byte & (1 << n) as u8 == 0;
            println!("Bit {} is {}", n, !bit_n);
            if bit_n {
                bit_vector.push(MBoolean::FALSE)
            }
            else {
                bit_vector.push(MBoolean::TRUE)
            }
        }
        // LE to BE
        bit_vector.reverse();
        byte_vector.push(bit_vector);
    }

    return match num_bits {
        8 => Number::int8(int8{bytes: byte_vector}),
        32 => Number::int32(int32{bytes: byte_vector}),
        _ => {panic!("{} bit number is not supported yet.")}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::byte::{make_empty_byte, make_byte};
    use crate::primitives::booleans::MBoolean;

    #[test]
    fn test_make_zero() {
        assert_eq!(int32 {bytes: vec![make_empty_byte(); 4]}, int32::make_zero());
    }

    #[test]
    fn test_is_valid32() {
        let valid_int32 = int32::make_zero();
        assert!(valid_int32.is_valid());
        let invalid_int32_too_small = int32 {
            bytes: vec![make_empty_byte(), make_empty_byte(), make_empty_byte()]
        };
        assert!(!invalid_int32_too_small.is_valid());
        let invalid_int32_too_big = int32 {
            bytes:  vec![make_empty_byte(), make_empty_byte(),
                         make_empty_byte(), make_empty_byte(), make_empty_byte()]
        };
        assert!(!invalid_int32_too_big.is_valid());
    }

    #[test]
    fn test_is_valid8() {
        let valid_int8 = int8::make_zero();
        assert!(valid_int8.is_valid());
        let invalid_int8_too_small = int32 {
            bytes: vec![]
        };
        assert!(!invalid_int8_too_small.is_valid());
        let invalid_int8_too_big = int32 {
            bytes:  vec![make_empty_byte(), make_empty_byte(), make_empty_byte()]
        };
        assert!(!invalid_int8_too_big.is_valid());
    }

    #[test]
    fn test_make_int8() {
        // b00000101 = 5
        let five_byte =
            make_byte_with_padding(vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE]);
        assert_eq!(
            int8::make_int8(&5),
            int8 {bytes: vec![five_byte]}
        );
    }

    #[test]
    fn test_make_int32() {
        let five_byte =
            make_byte_with_padding(vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE]);
        assert_eq!(
            int32::make_int32(&5),
            int32 {bytes: vec![make_empty_byte(), make_empty_byte(), make_empty_byte(), five_byte]}
        );
    }
}