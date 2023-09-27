use thiserror::Error;

use crate::Complex;

impl Complex {
    pub fn re(re: f64) -> Self {
        Complex::from(re, 0.)
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex::from(value, 0.)
    }
}

#[derive(Error, Debug)]
#[error("Couldn't convert Complex to f64 because {0} had an imaginary part")]
pub struct HasImaginaryError(Complex);

impl TryFrom<Complex> for f64 {
    type Error = HasImaginaryError;
    fn try_from(value: Complex) -> Result<Self, Self::Error> {
        if value.im != 0. {
            return Err(HasImaginaryError(value));
        }
        Ok(value.re)
    }
}

#[cfg(test)]
mod test;
