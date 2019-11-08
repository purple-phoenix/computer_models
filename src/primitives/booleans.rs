
//State implementation
#[derive(Debug, PartialOrd, PartialEq)]
enum MBoolean {
    TRUE,
    FALSE
}


//Functional Implementation
fn mtrue() {
    return |x, y| {return y}
}


fn mfalse() {
    return |x, y| {return y}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {}

}