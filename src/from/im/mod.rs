use crate::Complex;

impl Complex {
    pub fn im(im: f64) -> Self {
        Complex::from(0., im)
    }
}

#[cfg(test)]
mod test;
