use crate::complex;

#[test]
fn ops_mul() {
    let a = complex::Complex::from(1., 3.);
    let b = complex::Complex::from(1., 1.);
    assert_eq!(&a * &b, complex::Complex::from(-2., 4.));
    assert_eq!(a * b, complex::Complex::from(-2., 4.));
}

#[test]
fn ops_mul_float() {
    let a = complex::Complex::from(1., 3.);
    let b = 2.;
    assert_eq!(&a * &b, complex::Complex::from(2., 6.));
    assert_eq!(a * b, complex::Complex::from(2., 6.));
}
