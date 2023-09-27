use crate::Complex;

#[test]
fn ops_mul() {
    let a = Complex::from(1., 3.);
    let b = Complex::from(1., 1.);
    assert_eq!(&a * &b, Complex::from(-2., 4.));
    assert_eq!(a * b, Complex::from(-2., 4.));
}

#[test]
fn ops_mul_float() {
    let a = Complex::from(1., 3.);
    let b = 2.;
    assert_eq!(&a * &b, Complex::from(2., 6.));
    assert_eq!(&b * &a, Complex::from(2., 6.));
    assert_eq!(a * b, Complex::from(2., 6.));
}
