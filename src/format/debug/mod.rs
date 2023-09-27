use crate::Complex;

impl core::fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.3}{}{:.3}i ({:.3}∠{:.3})",
            self.re,
            if self.im >= 0. { "+" } else { "-" },
            self.im.abs(),
            self.abs(),
            self.arg()
        )
    }
}

#[cfg(test)]
mod test;
