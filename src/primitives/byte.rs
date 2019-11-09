use crate::primitives::booleans::MBoolean;

type MByte = Vec<MBoolean>;

pub fn make_empty_byte() -> MByte {
    return vec![MBoolean::FALSE; 8]
}

pub fn make_byte(maybe_byte: Vec<MBoolean>) -> Option<MByte> {
    if maybe_byte.len() == 8 {
        return Some(maybe_byte);
    }
    else {
        return None;
    }
}

pub fn make_byte_with_padding(byte: Vec<MBoolean>) -> MByte {
    if byte.len() == 0 {
        return make_empty_byte();
    }
    let byte_len = 8;
    let num_zeroes_to_pad = byte_len - byte.len();
    let mut return_byte = Vec::with_capacity(byte_len);
    for x in 0..num_zeroes_to_pad{
        return_byte.push(MBoolean::FALSE);
    }
    for x in 0..byte.len() {
        return_byte.push(byte[x]);
    }


    return return_byte;
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

    #[test]
    fn test_make_byte() {
        assert_eq!(Some(make_empty_byte()), make_byte(make_empty_byte()));
        assert_eq!(None, make_byte(vec![MBoolean::FALSE, MBoolean::TRUE]))
    }

    #[test]
    fn test_make_byte_with_padding() {
        assert_eq!(
            vec![MBoolean::FALSE, MBoolean::FALSE, MBoolean::FALSE, MBoolean::TRUE,
                MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE
        ],
            make_byte_with_padding(
                vec![MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE, MBoolean::TRUE])
        );
        assert_eq!(make_empty_byte(), make_byte_with_padding(make_empty_byte()));
        assert_eq!(make_empty_byte(), make_byte_with_padding(vec![]))
    }
}