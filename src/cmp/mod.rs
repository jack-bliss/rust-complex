use crate::Complex;

impl PartialEq<f64> for Complex {
    fn eq(&self, other: &f64) -> bool {
        self.im == 0. && &self.re == other
    }
}
impl PartialEq<Complex> for f64 {
    fn eq(&self, other: &Complex) -> bool {
        other.im == 0. && &other.re == self
    }
}

#[cfg(test)]
mod test;
