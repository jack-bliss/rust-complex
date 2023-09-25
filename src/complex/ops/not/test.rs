use crate::complex;

#[test]
fn ops_not() {
    assert_eq!(
        !complex::Complex::from(1., 1.),
        complex::Complex::from(1., -1.)
    )
}
