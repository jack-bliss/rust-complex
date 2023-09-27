use thiserror::Error;

use crate::Complex;

#[derive(Error, Debug)]
#[error("Couldn't convert Complex to f64 because {0} had an imaginary part")]
pub struct HasImaginaryError(Complex);

impl TryFrom<Complex> for f64 {
    type Error = HasImaginaryError;
    fn try_from(value: Complex) -> Result<Self, Self::Error> {
        value.to_float()
    }
}

impl Complex {
    pub fn to_float(self) -> Result<f64, HasImaginaryError> {
        if self.im != 0. {
            return Err(HasImaginaryError(self));
        }
        Ok(self.re)
    }
}
