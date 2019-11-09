use crate::primitives::byte::{MByte, make_empty_byte};

#[derive(Debug, PartialOrd, PartialEq)]
struct int32 {
    bytes: Vec<MByte>
}

impl int32 {

    fn make_zero() -> int32 {
        return int32 {bytes: vec![make_empty_byte(); 4]}
    }

    fn is_valid(&self) -> bool {
        self.bytes.len() == 4
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
    fn test_is_valid() {
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
}