use crate::Complex;

impl Complex {
    pub fn theta(theta: f64) -> Self {
        Complex::from_polar(1., theta)
    }
}

#[cfg(test)]
mod test;
