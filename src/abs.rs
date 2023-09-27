use crate::Complex;

impl Complex {
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test_absolute_value {
    use crate::Complex;
    #[test]
    fn should_get_absolute_value_for_complex_numbers() {
        let a = Complex::from(3., 4.);
        assert_eq!(a.abs(), 5.);
        let b = Complex::from(5., 0.);
        assert_eq!(b.abs(), 5.);
    }
}
