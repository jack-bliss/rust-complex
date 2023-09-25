use crate::complex::Complex;

impl Complex {
    pub fn from(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

#[cfg(test)]
mod test;
