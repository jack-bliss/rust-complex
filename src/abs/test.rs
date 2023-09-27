use crate::Complex;

#[test]
fn abs() {
    let a = Complex::from(3., 4.);
    assert_eq!(a.abs(), 5.);
    let b = Complex::from(5., 0.);
    assert_eq!(b.abs(), 5.);
}
