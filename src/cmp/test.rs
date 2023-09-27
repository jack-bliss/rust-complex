use crate::Complex;

#[test]
fn cmp_f64() {
    assert_eq!(Complex::unit(), 1.);
    assert_eq!(Complex::from(5.5, 0.), 5.5);
    assert_eq!(5.5, Complex::from(5.5, 0.));
    assert_ne!(Complex::from(1., 1.,), 1.);
}
