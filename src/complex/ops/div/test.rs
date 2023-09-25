use crate::complex;

#[test]
fn ops_div() {
    let a = complex::Complex::from(4., 4.);
    let b = complex::Complex::from(2., 2.);
    assert_eq!(&a / &b, complex::Complex::from(2., 0.));
    assert_eq!(a / b, complex::Complex::from(2., 0.));
}

#[test]
fn ops_div_float() {
    let a = complex::Complex::from(4., 4.);
    let b = 2.;
    assert_eq!(&a / &b, complex::Complex::from(2., 2.));
    assert_eq!(a / b, complex::Complex::from(2., 2.));
}
