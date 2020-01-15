use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) -> bool {
    assert_eq!("Hello", s.borrow());
    true
}

#[test]
fn test_basics() {
    let s = "Hello".to_string();

    check(s);

    let s = "Hello";

    let flag = check(s);
    assert_eq!(true, flag);
}
