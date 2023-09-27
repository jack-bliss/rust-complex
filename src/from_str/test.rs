use crate::Complex;
use std::str::FromStr;

#[test]
fn from_str() {
    let z1 = Complex::from_str("1+i").unwrap();
    assert_eq!(z1, Complex::from(1., 1.));
    let z2 = Complex::from_str("1-i").unwrap();
    assert_eq!(z2, Complex::from(1., -1.));
    let z3 = Complex::from_str("-1+1i").unwrap();
    assert_eq!(z3, Complex::from(-1., 1.));
    let z4 = Complex::from_str("-1-1i").unwrap();
    assert_eq!(z4, Complex::from(-1., -1.));
    let z5 = Complex::from_str("1.3+2.2i").unwrap();
    assert_eq!(z5, Complex::from(1.3, 2.2));
    let z6 = Complex::from_str("1.3-2.2i").unwrap();
    assert_eq!(z6, Complex::from(1.3, -2.2));
}
