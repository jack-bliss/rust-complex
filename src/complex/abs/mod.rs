use super::Complex;

impl Complex {
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test;
