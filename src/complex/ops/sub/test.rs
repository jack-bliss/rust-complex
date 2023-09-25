use crate::complex;

#[test]
fn ops_sub() {
    let a = complex::Complex::from(2., 3.);
    let b = complex::Complex::from(1., 1.);
    assert_eq!(&a - &b, complex::Complex::from(1., 2.));
    assert_eq!(a - b, complex::Complex::from(1., 2.));
}
