#[derive(PartialEq, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

mod abs;
mod arg;
#[cfg(test)]
mod assert;
mod compare;
mod format;
mod from;
mod ops;
mod pow;
mod roots_of_unity;
mod to;
