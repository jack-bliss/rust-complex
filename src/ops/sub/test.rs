use crate::Complex;

#[test]
fn ops_sub() {
    let a = Complex::from(2., 3.);
    let b = Complex::from(1., 1.);
    assert_eq!(&a - &b, Complex::from(1., 2.));
    assert_eq!(a - b, Complex::from(1., 2.));
}
