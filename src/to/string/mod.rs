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

#[cfg(test)]
mod test;
