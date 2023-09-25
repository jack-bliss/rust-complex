use crate::complex::{assert, Complex};

#[test]
fn pow() {
    assert::close(&Complex::from(2., 0.).pow(2.), &Complex::from(4., 0.));
    assert::close(&Complex::from(3., 4.).pow(2.), &Complex::from(-7., 24.));
}
