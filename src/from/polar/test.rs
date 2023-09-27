use crate::{assert, Complex};
use std::f64::consts::{PI, TAU};

#[test]
fn from_polar() {
    assert_eq!(Complex::from_polar(3., 0.), Complex { re: 3., im: 0. });
    assert::close(
        &Complex::from_polar(2., PI / 4.),
        &Complex {
            re: 2_f64.sqrt(),
            im: 2_f64.sqrt(),
        },
    );
    assert::close(&Complex::from_polar(3., 0.), &Complex::from_polar(3., TAU));
    assert::close(&Complex::from_polar(3., 0.), &Complex::from_polar(-3., PI));
}
