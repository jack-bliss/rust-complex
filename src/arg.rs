use crate::Complex;

impl Complex {
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }
}

#[cfg(test)]
mod test_argument {
    use crate::Complex;
    use std::f64::consts::PI;

    #[test]
    fn should_get_angle_from_x_axis_for_complex_number() {
        [
            // complex number, expected argument
            (Complex::from(1., 0.), 0.),
            (Complex::from(5., 0.), 0.),
            (Complex::from(0., 3.), PI / 2.),
            (Complex::from(-1., 0.), PI),
            (Complex::from(1., 1.), PI / 4.),
            (Complex::from(0., -1.), PI / -2.),
        ]
        .iter()
        .for_each(|(z, a)| assert_eq!(&z.arg(), a));
    }
}
