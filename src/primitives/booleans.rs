use crate::primitives::booleans::MBoolean::{TRUE, FALSE};

// State implementation
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum MBoolean {
    TRUE,
    FALSE
}

pub fn make_true_st() -> MBoolean {
    return TRUE;
}

pub fn make_false_st() -> MBoolean {
    return FALSE;
}

// Functional Implementation TODO

pub type MTrue<T1, T2> = Box<dyn Fn(T1, T2) -> T1>;

pub type MFalse<T1, T2> = Box<dyn Fn(T1, T2) -> T2>;

pub enum MBooleanFn<T1, T2> {
    TRUE(MTrue<T1, T2>),
    FALSE(MFalse<T1, T2>)
}

pub fn make_true_fn<T1, T2>() -> MTrue<T1, T2> {
    return Box::new(|x, y| {x});
}

pub fn make_false_fn<T1, T2>() -> MFalse<T1, T2> {
    return Box::new(|x, y| {y})
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_fn() {
        let instance_of_true: MTrue<i32, i32> = make_true_fn();
        let instance_of_false: MTrue<i32, i32> = make_false_fn();
        assert_eq!(1, instance_of_true(1, 2));
        assert_eq!(2, instance_of_false(1, 2))
    }

    #[test]
    fn test_bool_st() {
        assert_eq!(TRUE, make_true_st());
        assert_eq!(FALSE, make_false_st());
    }

}