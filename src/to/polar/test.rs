use crate::{to::polar::PolarForm, Complex};
use std::f64::consts::PI;

#[test]
fn to_polar() {
    assert_eq!(
        Complex::from(1., 1.).to_polar(),
        PolarForm {
            abs: 2_f64.sqrt(),
            arg: PI / 4.
        }
    );
}
