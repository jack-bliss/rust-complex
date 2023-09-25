use crate::complex::Complex;

#[test]
fn format_debug() {
    assert_eq!(
        format!("{:?}", Complex::unit()),
        String::from("1.000+0.000i (1.000∠0.000)")
    )
}
