use super::super::Complex;

impl core::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.2}{}{:.2}i",
            self.re,
            if self.im > 0. { "+" } else { "-" },
            self.im.abs()
        )
    }
}
