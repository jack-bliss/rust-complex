use crate::{to::polar::PolarForm, Complex};

pub trait Power<T> {
    fn pow(&self, index: T) -> Self;
}

impl Power<f64> for Complex {
    fn pow(&self, ind: f64) -> Self {
        let PolarForm { abs, arg } = self.to_polar();
        Complex::from_polar(abs.powf(ind), arg * ind)
    }
}

impl Power<u128> for Complex {
    fn pow(&self, ind: u128) -> Self {
        match ind {
            0 => Complex::unit(),
            1 => self.clone(),
            ind => (1..ind).fold(self.clone(), |acc, _| acc * self),
        }
    }
}

#[cfg(test)]
mod test;
