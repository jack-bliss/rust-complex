use crate::complex::{self, assert};

#[test]
fn from_polar() {
    assert_eq!(
        complex::Complex::from_polar(3., 0.),
        complex::Complex { re: 3., im: 0. }
    );
    assert::close(
        &complex::Complex::from_polar(2., std::f64::consts::PI / 4.),
        &complex::Complex {
            re: 2_f64.sqrt(),
            im: 2_f64.sqrt(),
        },
    );
    assert::close(
        &complex::Complex::from_polar(3., 0.),
        &complex::Complex::from_polar(3., std::f64::consts::PI * 2.),
    );
    assert::close(
        &complex::Complex::from_polar(3., 0.),
        &complex::Complex::from_polar(-3., std::f64::consts::PI),
    );
}
