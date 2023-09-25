use crate::complex;

#[test]
fn from_cartes() {
    assert_eq!(
        complex::Complex::from(2., 2.),
        complex::Complex { re: 2., im: 2. }
    )
}
