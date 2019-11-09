use crate::primitives::byte::{MByte, make_empty_byte};

pub enum Number {
    int8(int8),
    int32(int32)
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct int32 {
    bytes: Vec<MByte>
}

#[derive(Debug, PartialOrd, PartialEq)]
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
}

impl int8 {

    pub fn make_zero() -> int8 {
        return int8 {bytes: vec![make_empty_byte()]}
    }

    pub fn is_valid(&self) -> bool {
        self.bytes.len() == 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::byte::make_empty_byte;

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
}