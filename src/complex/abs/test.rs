use crate::complex;

#[test]
fn abs() {
    let a = complex::Complex::from(3., 4.);
    assert_eq!(a.abs(), 5.);
    let b = complex::Complex::from(5., 0.);
    assert_eq!(b.abs(), 5.);
}
