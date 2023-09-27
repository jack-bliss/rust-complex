use crate::Complex;
use std::f64::consts::PI;

#[test]
fn arg() {
    let a = Complex::from(1., 0.);
    assert_eq!(a.arg(), 0., "arg wrong for {a}");

    let b = Complex::from(5., 0.);
    assert_eq!(b.arg(), 0., "arg wrong for {b}");

    let c = Complex::from(0., 3.);
    assert_eq!(c.arg(), PI / 2., "arg wrong for {c}");

    let d = Complex::from(-1., 0.);
    assert_eq!(d.arg(), PI, "arg wrong for {d}");

    let e = Complex::from(1., 1.);
    assert_eq!(e.arg(), PI / 4., "arg wrong for {e}");

    let f = Complex::from(0., -1.);
    assert_eq!(f.arg(), PI / -2., "arg wrong for {e}");
}
