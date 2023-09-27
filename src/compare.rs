use crate::Complex;

impl PartialEq<f64> for Complex {
    fn eq(&self, other: &f64) -> bool {
        self.im == 0. && &self.re == other
    }
}

impl PartialEq<Complex> for f64 {
    fn eq(&self, other: &Complex) -> bool {
        other.im == 0. && &other.re == self
    }
}

impl PartialEq<Complex> for String {
    fn eq(&self, other: &Complex) -> bool {
        &other.to_string() == self
    }
}

impl PartialEq<String> for Complex {
    fn eq(&self, other: &String) -> bool {
        &self.to_string() == other
    }
}

impl PartialEq<Complex> for &str {
    fn eq(&self, other: &Complex) -> bool {
        &other.to_string() == self
    }
}

impl PartialEq<&str> for Complex {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == String::from(*other)
    }
}

#[cfg(test)]
mod test_compare_to_float {
    use crate::Complex;

    #[test]
    fn should_not_be_equal_with_imaginary_part() {
        let z = Complex::from(2., 1.);
        assert_ne!(z, 2.);
        assert_ne!(2., z);
    }

    #[test]
    fn should_be_equal_without_imaginary_part() {
        let z = Complex::from(2., 0.);
        assert_eq!(z, 2.);
        assert_eq!(2., z);
    }

    #[test]
    fn should_be_equal_to_string() {
        assert_eq!(Complex::from(1., 1.), String::from("1+1i"));
    }

    #[test]
    fn should_be_equal_to_str() {
        assert_eq!(Complex::from(1., 1.), "1+1i");
        assert_eq!("1+1i", Complex::from(1., 1.));
    }
}
