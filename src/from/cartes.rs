use crate::Complex;

impl Complex {
    pub const fn from(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

#[cfg(test)]
mod test_from_cartesian {
    use crate::Complex;

    #[test]
    fn should_create_a_complex_number() {
        assert_eq!(Complex::from(2., 2.), Complex { re: 2., im: 2. })
    }
}
