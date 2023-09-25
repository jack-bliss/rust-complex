use crate::complex;

#[test]
fn arg() {
    let a = complex::Complex::from(1., 0.);
    assert_eq!(a.arg(), 0., "arg wrong for {a}");

    let b = complex::Complex::from(5., 0.);
    assert_eq!(b.arg(), 0., "arg wrong for {b}");

    let c = complex::Complex::from(0., 3.);
    assert_eq!(c.arg(), std::f64::consts::PI / 2., "arg wrong for {c}");

    let d = complex::Complex::from(-1., 0.);
    assert_eq!(d.arg(), std::f64::consts::PI, "arg wrong for {d}");

    let e = complex::Complex::from(1., 1.);
    assert_eq!(e.arg(), std::f64::consts::PI / 4., "arg wrong for {e}");

    let f = complex::Complex::from(0., -1.);
    assert_eq!(f.arg(), std::f64::consts::PI / -2., "arg wrong for {e}");
}
