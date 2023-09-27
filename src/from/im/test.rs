use crate::Complex;

#[test]
fn from_re() {
    assert_eq!(Complex::im(3.), Complex::from(0., 3.));
}
