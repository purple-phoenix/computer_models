use crate::primitives::booleans::MBoolean;

type MByte = Vec<MBoolean>;

pub fn make_empty_byte() -> MByte {
    return vec![MBoolean::FALSE; 8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_make_empty_byte() {
        assert_eq!(vec![MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE,
                        MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE
        ], make_empty_byte())
    }
}