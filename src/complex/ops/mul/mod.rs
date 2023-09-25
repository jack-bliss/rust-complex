use impl_ops::*;
use std::ops;

use super::super::Complex;

impl_op_ex!(*|lhs: &Complex, rhs: &Complex| -> Complex {
    let re = (lhs.re * rhs.re) - (lhs.im * rhs.im);
    let im = (lhs.re * rhs.im) + (lhs.im * rhs.re);
    Complex::from(re, im)
});

impl_op_ex!(*|lhs: &Complex, rhs: &f64| -> Complex { Complex::from(lhs.re * rhs, lhs.im * rhs) });

#[cfg(test)]
mod test;
