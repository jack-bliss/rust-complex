use crate::Complex;

#[test]
fn from_re() {
    assert_eq!(Complex::re(3.), Complex::from(3., 0.));
}
