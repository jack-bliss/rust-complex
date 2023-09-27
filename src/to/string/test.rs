use crate::Complex;

#[test]
fn to_string() {
    assert_eq!(String::from("1+1i"), Complex::from(1., 1.).to_string());
    assert_eq!(String::from("1-1i"), Complex::from(1., -1.).to_string());
    let z = Complex::from(1., 1.);
    let str = (&z).to_string();
    assert_eq!(String::from("1+1i"), str);
}
