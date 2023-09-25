use crate::complex;

#[test]
fn ops_add() {
    let a = complex::Complex::from(1., 1.);
    let b = complex::Complex::from(2., 3.);
    assert_eq!(&a + &b, complex::Complex::from(3., 4.));
    assert_eq!(a + b, complex::Complex::from(3., 4.));
}
