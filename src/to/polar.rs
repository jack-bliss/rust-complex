use crate::Complex;

#[derive(PartialEq, Debug)]
pub struct PolarForm {
    pub abs: f64,
    pub arg: f64,
}

impl Complex {
    pub fn to_polar(&self) -> PolarForm {
        let abs = self.abs();
        let arg = self.arg();
        PolarForm { abs, arg }
    }
}

#[cfg(test)]
mod test_to_polar_form {
    use crate::{to::polar::PolarForm, Complex};
    use std::f64::consts::PI;

    #[test]
    fn should_create_polar_form() {
        assert_eq!(
            Complex::from(1., 1.).to_polar(),
            PolarForm {
                abs: 2_f64.sqrt(),
                arg: PI / 4.
            }
        );
    }
}
