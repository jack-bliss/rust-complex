use std::f64::consts::TAU;

use crate::Complex;

impl Complex {
    pub fn roots_of_unity(roots: u8) -> Vec<Complex> {
        (0..roots)
            .map(|root| {
                let arg = TAU * (root as f64) / (roots as f64);
                Complex::from_polar(1., arg)
            })
            .collect()
    }
}

#[cfg(test)]
mod test_generating_roots_of_unity {
    use std::f64::consts::TAU;

    use crate::{assert, pow::Power, Complex};

    #[test]
    fn should_create_single_root_of_unity() {
        let one_root = Complex::roots_of_unity(1);
        assert_eq!(one_root.len(), 1);
        assert_eq!(one_root[0], Complex::unit(), "one root wrong");
    }

    #[test]
    fn should_create_two_roots_of_unity() {
        let two_root: Vec<Complex> = Complex::roots_of_unity(2);
        assert_eq!(two_root.len(), 2);
        assert::close(&two_root[0], &Complex::unit());
        assert::close(&two_root[1], &Complex::from(-1., 0.));
    }

    #[test]
    fn should_create_three_roots_of_unity() {
        let three_root = Complex::roots_of_unity(3);
        assert_eq!(three_root.len(), 3);
        assert::close(&three_root[0], &Complex::unit());
        assert::close(&three_root[1], &Complex::from_polar(1., TAU * (1. / 3.)));
        assert::close(&three_root[2], &Complex::from_polar(1., TAU * (2. / 3.)));
    }

    #[test]
    fn should_create_five_roots_of_unity_which_raised_to_fifth_power_equal_one() {
        let five_root = Complex::roots_of_unity(5);
        five_root.iter().for_each(|root| {
            // taking root to the fifth power should be close to 1
            assert::close(
                &root.pow(5),     // root to 5th power
                &Complex::unit(), // 1 in complex form
            )
        })
    }
}
