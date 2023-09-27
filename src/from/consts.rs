use crate::Complex;

impl Complex {
    pub const fn zero() -> Self {
        Complex::from(0., 0.)
    }
    pub const fn unit() -> Self {
        Complex::from(1., 0.)
    }
}
