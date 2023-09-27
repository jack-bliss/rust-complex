use crate::Complex;

impl Complex {
    pub fn re(re: f64) -> Self {
        Complex::from(re, 0.)
    }
}

#[cfg(test)]
mod test;
