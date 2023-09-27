#[derive(PartialEq, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

mod abs;
mod arg;
#[cfg(test)]
mod assert;
mod cmp;
mod format;
mod from;
mod from_str;
mod ops;
mod pow;
mod roots_of_unity;
mod to;

impl Complex {
    pub fn print_abs(&self) {
        println!("|{self}| = {:.2}", self.abs());
    }
}
