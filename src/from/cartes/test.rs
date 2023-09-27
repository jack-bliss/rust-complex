use crate::Complex;

#[test]
fn from_cartes() {
    assert_eq!(Complex::from(2., 2.), Complex { re: 2., im: 2. })
}
