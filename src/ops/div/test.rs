use crate::Complex;

#[test]
fn ops_div() {
    let a = Complex::from(4., 4.);
    let b = Complex::from(2., 2.);
    assert_eq!(&a / &b, Complex::from(2., 0.));
    assert_eq!(a / b, Complex::from(2., 0.));
}

#[test]
fn ops_div_float() {
    let a = Complex::from(4., 4.);
    let b = 2.;
    assert_eq!(&a / &b, Complex::from(2., 2.));
    assert_eq!(&b / &a, Complex::from(0.25, -0.25));
    assert_eq!(a / b, Complex::from(2., 2.));
    assert_eq!(1. / Complex::from(3., 4.), Complex::from(0.12, -0.16));
}
