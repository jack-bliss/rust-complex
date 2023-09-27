use crate::{assert, pow::Power, Complex};

#[test]
fn pow() {
    assert::close(&Complex::from(2., 0.).pow(2.), &Complex::from(4., 0.));
    assert::close(&Complex::from(3., 4.).pow(2.), &Complex::from(-7., 24.));
}

#[test]
fn pow_ind() {
    assert_eq!(Complex::from(3., 6.).pow(0), Complex::unit());
    assert_eq!(Complex::from(3., 6.).pow(1), Complex::from(3., 6.));
    assert::close(&Complex::from(2., 0.).pow(2), &Complex::from(4., 0.));
    assert::close(&Complex::from(2., 0.).pow(3), &Complex::from(8., 0.));
    assert::close(&Complex::from(1., 1.).pow(2), &Complex::from(0., 2.));
    assert::close(&Complex::from(1., 1.).pow(3), &Complex::from(-2., 2.));
}
