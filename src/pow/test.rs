use crate::{assert, Complex};

#[test]
fn pow() {
    assert::close(&Complex::from(2., 0.).pow(2.), &Complex::from(4., 0.));
    assert::close(&Complex::from(3., 4.).pow(2.), &Complex::from(-7., 24.));
}

#[test]
fn pow_ind() {
    assert_eq!(Complex::from(3., 6.).powi(0), Complex::unit());
    assert_eq!(Complex::from(3., 6.).powi(1), Complex::from(3., 6.));
    assert::close(&Complex::from(2., 0.).powi(2), &Complex::from(4., 0.));
    assert::close(&Complex::from(2., 0.).powi(3), &Complex::from(8., 0.));
    assert::close(&Complex::from(1., 1.).powi(2), &Complex::from(0., 2.));
    assert::close(&Complex::from(1., 1.).powi(3), &Complex::from(-2., 2.));
}
