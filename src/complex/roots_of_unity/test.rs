use std::f64::consts::PI;

use crate::complex::{assert, Complex};

#[test]
fn roots_of_unity() {
    let one_root = Complex::roots_of_unity(1);
    assert_eq!(one_root.len(), 1);
    assert_eq!(one_root[0], Complex::from(1., 0.), "one root wrong");

    let two_root: Vec<Complex> = Complex::roots_of_unity(2);
    assert_eq!(two_root.len(), 2);
    assert::close(&two_root[0], &Complex::from(1., 0.));
    assert::close(&two_root[1], &Complex::from(-1., 0.));

    let three_root = Complex::roots_of_unity(3);
    assert_eq!(three_root.len(), 3);
    assert::close(&three_root[0], &Complex::from(1., 0.));
    assert::close(&three_root[1], &Complex::from_polar(1., PI * (2. / 3.)));
    assert::close(&three_root[2], &Complex::from_polar(1., PI * (4. / 3.)));

    let five_root = Complex::roots_of_unity(5);
    five_root.iter().for_each(|root| {
        // taking root to the fifth power should be close to 1
        assert::close(
            &root.pow(5.),          // root to 5th power
            &Complex::from(1., 0.), // 1 in complex form
        )
    })
}
