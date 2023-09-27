use crate::Complex;

impl Complex {
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}i",
            self.re,
            if self.im >= 0. { "+" } else { "-" },
            self.im.abs()
        )
    }
}

impl From<Complex> for String {
    fn from(value: Complex) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod test_to_string {
    use crate::Complex;

    fn s(str: &str) -> String {
        String::from(str)
    }

    #[test]
    fn should_return_the_correct_string() {
        assert_eq!(s("1+1i"), Complex::from(1., 1.).to_string());
        assert_eq!(s("1-1i"), Complex::from(1., -1.).to_string());
        let z = Complex::from(1., 1.);
        let str = (&z).to_string();
        assert_eq!(s("1+1i"), str);
    }
}
