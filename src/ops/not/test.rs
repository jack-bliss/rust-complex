use crate::Complex;

#[test]
fn ops_not() {
    assert_eq!(!Complex::from(1., 1.), Complex::from(1., -1.))
}
