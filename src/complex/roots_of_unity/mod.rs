use std::f64::consts::PI;

use crate::complex::Complex;

impl Complex {
    pub fn roots_of_unity(roots: u8) -> Vec<Complex> {
        (0..roots)
            .map(|root| Complex::from_polar(1., PI * 2. * (root as f64) / (roots as f64)))
            .collect()
    }
}

#[cfg(test)]
mod test;
