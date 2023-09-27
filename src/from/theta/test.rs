use std::f64::consts::PI;

use crate::{assert, Complex};

#[test]
fn from_theta() {
    let z1 = Complex::theta(0.);
    assert_eq!(z1, Complex::unit());
    let z2 = Complex::theta(PI);
    assert::close(&z2, &Complex::re(-1.));
    let z3 = Complex::theta(PI / 2.);
    assert::close(&z3, &Complex::im(1.));
    let z4 = Complex::theta(PI / -2.);
    assert::close(&z4, &Complex::im(-1.));
}
