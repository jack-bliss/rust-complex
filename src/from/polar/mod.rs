use crate::Complex;

impl Complex {
    pub fn from_polar(abs: f64, arg: f64) -> Self {
        Self {
            re: abs * arg.cos(),
            im: abs * arg.sin(),
        }
    }
}

#[cfg(test)]
mod test;
