use crate::Complex;

impl Complex {
    pub const fn re(re: f64) -> Self {
        Complex::from(re, 0.)
    }
    pub const fn im(im: f64) -> Self {
        Complex::from(0., im)
    }
    pub fn theta(theta: f64) -> Self {
        Complex::from_polar(1., theta)
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex::from(value, 0.)
    }
}
