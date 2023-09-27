use crate::Complex;

#[test]
fn to_string() {
    assert_eq!(String::from("1+1i"), Complex::from(1., 1.).to_string());
    assert_eq!(String::from("1-1i"), Complex::from(1., -1.).to_string());
}
