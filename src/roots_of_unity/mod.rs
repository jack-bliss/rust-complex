use std::f64::consts::TAU;

use crate::Complex;

impl Complex {
    pub fn roots_of_unity(roots: u8) -> Vec<Complex> {
        (0..roots)
            .map(|root| {
                let arg = TAU * (root as f64) / (roots as f64);
                Complex::from_polar(1., arg)
            })
            .collect()
    }
}

#[cfg(test)]
mod test;
