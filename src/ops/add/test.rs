use crate::Complex;

#[test]
fn ops_add() {
    let a = Complex::from(1., 1.);
    let b = Complex::from(2., 3.);
    assert_eq!(&a + &b, Complex::from(3., 4.));
    assert_eq!(a + b, Complex::from(3., 4.));
}
