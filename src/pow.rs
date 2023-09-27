use crate::{to::polar::PolarForm, Complex};

pub trait Power<T> {
    fn pow(&self, index: T) -> Self;
}

impl Power<f64> for Complex {
    fn pow(&self, ind: f64) -> Self {
        let PolarForm { abs, arg } = self.to_polar();
        Complex::from_polar(abs.powf(ind), arg * ind)
    }
}

impl Power<u128> for Complex {
    fn pow(&self, ind: u128) -> Self {
        match ind {
            0 => Complex::unit(),
            1 => self.clone(),
            ind => (1..ind).fold(self.clone(), |acc, _| acc * self),
        }
    }
}

#[cfg(test)]
mod test_raising_to_power {
    use crate::{assert, pow::Power, Complex};

    #[test]
    fn should_raise_to_a_floating_point_power() {
        assert::close(&Complex::from(2., 0.).pow(2.), &Complex::from(4., 0.));
        assert::close(&Complex::from(3., 4.).pow(2.), &Complex::from(-7., 24.));
    }

    #[test]
    fn should_return_unit_when_raised_to_integer_0() {
        assert_eq!(Complex::from(3., 6.).pow(0), 1.);
    }

    #[test]
    fn should_return_self_when_raised_to_integer_one() {
        assert_eq!(Complex::from(3., 6.).pow(1), Complex::from(3., 6.));
    }

    #[test]
    fn should_raise_to_integer_power() {
        assert::close(&Complex::from(2., 0.).pow(2), &Complex::from(4., 0.));
        assert::close(&Complex::from(2., 0.).pow(3), &Complex::from(8., 0.));
        assert::close(&Complex::from(1., 1.).pow(2), &Complex::from(0., 2.));
        assert::close(&Complex::from(1., 1.).pow(3), &Complex::from(-2., 2.));
    }
}
