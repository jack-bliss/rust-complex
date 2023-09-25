use super::{to::polar::PolarForm, Complex};

impl Complex {
    pub fn pow(&self, ind: f64) -> Self {
        let PolarForm { abs, arg } = self.to_polar();
        Complex::from_polar(abs.powf(ind), arg * ind)
    }
}

#[cfg(test)]
mod test;
