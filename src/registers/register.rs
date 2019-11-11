use crate::registers::gated_latch::GatedLatch;
use crate::primitives::booleans::MBoolean;
use crate::primitives::byte::MByte;

pub trait StoresBytes {
    fn read_bytes(&self) -> Vec<MByte>;
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Register {
    num_bytes: usize,
    data: Vec<Vec<GatedLatch>>
}

impl Register {
    fn make_empty_register(num_bytes: usize) -> Register {
        let mut data = Vec::with_capacity(num_bytes);
        let byte_len = 8;
        for _ in 0..num_bytes {
            let mut latch_row = Vec::with_capacity(byte_len);
            for _ in 0..byte_len {
                latch_row.push(GatedLatch::new(MBoolean::FALSE));
            }
            data.push(latch_row);
        }
        return Register {num_bytes, data};
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_empty_register() {
        let byte_len = 8;

        let eight_bit_empty_register_data
            = vec![vec![GatedLatch::new(MBoolean::FALSE); byte_len]];
        assert_eq!(Register::make_empty_register(1),
                   Register{num_bytes: 1, data: eight_bit_empty_register_data});

        let thirty_two_bit_empty_register_data
            = vec![vec![GatedLatch::new(MBoolean::FALSE); byte_len]; 4];
        assert_eq!(Register::make_empty_register(4),
                   Register{num_bytes:4, data:thirty_two_bit_empty_register_data});

    }

}