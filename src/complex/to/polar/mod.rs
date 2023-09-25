use crate::complex::Complex;

#[derive(PartialEq, Debug)]
pub struct PolarForm {
    pub abs: f64,
    pub arg: f64,
}

impl Complex {
    pub fn to_polar(&self) -> PolarForm {
        let abs = self.abs();
        let arg = self.arg();
        PolarForm { abs, arg }
    }
}

#[cfg(test)]
mod test;
