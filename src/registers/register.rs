use crate::registers::gated_latch::{GatedLatch, read_latch_row_to_byte};
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

    fn make_register(data: Vec<Vec<GatedLatch>>) -> Register {
        return Register{num_bytes: data.len(), data}
    }
}

impl StoresBytes for Register {
    fn read_bytes(&self) -> Vec<MByte> {
        let mut byte_vector = Vec::with_capacity(self.num_bytes);
        for row_index in 0..self.num_bytes {
            let latch_row = self.data[row_index].clone();
            byte_vector.push(read_latch_row_to_byte(latch_row));
        }
        return byte_vector;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::resume_unwind;
    use crate::primitives::byte::make_byte_with_padding;

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

    #[test]
    fn test_read_register_bytes() {
        let mut register_data_byte = vec![GatedLatch::new(MBoolean::FALSE); 5];
        register_data_byte.push(GatedLatch::new(MBoolean::TRUE));
        register_data_byte.push(GatedLatch::new(MBoolean::FALSE));
        register_data_byte.push(GatedLatch::new(MBoolean::TRUE));

        let register = Register::make_register(vec![register_data_byte]);

        assert_eq!(*register.read_bytes().get(0).unwrap(),
                   make_byte_with_padding(
                       vec![MBoolean::TRUE, MBoolean::FALSE, MBoolean::TRUE]));

    }

}