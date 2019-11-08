use crate::primitives::booleans::MBoolean::{TRUE, FALSE};

//State implementation
#[derive(Debug, PartialOrd, PartialEq)]
enum MBoolean {
    TRUE,
    FALSE
}

fn make_true_st() -> MBoolean {
    return TRUE;
}

fn make_false_st() -> MBoolean {
    return FALSE;
}

type MTrue<T1, T2> = Box<Fn(T1, T2) -> T1>;

type MFalse<T1, T2> = Box<Fn(T1, T2) -> T2>;

fn make_true_fn<T1, T2>() -> MTrue<T1, T2> {
    return Box::new(|x, y| {x});
}

fn make_false_fn<T1, T2>() -> MFalse<T1, T2> {
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