use crate::complex::{to::polar::PolarForm, Complex};

#[test]
fn to_polar() {
    assert_eq!(
        Complex::from(1., 1.).to_polar(),
        PolarForm {
            abs: 2_f64.sqrt(),
            arg: std::f64::consts::PI / 4.
        }
    );
}
