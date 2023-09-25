use super::Complex;

impl Complex {
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }
}

#[cfg(test)]
mod test;
