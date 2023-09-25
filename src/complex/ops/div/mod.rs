use impl_ops::*;
use std::ops;

use super::super::Complex;

impl_op_ex!(/|lhs: &Complex, rhs: &Complex| -> Complex {
    let den = rhs.re.powi(2) + rhs.im.powi(2);
    let re_num = (lhs.re * rhs.re) + (lhs.im * rhs.im);
    let im_num = (lhs.im * rhs.re) - (lhs.re * rhs.im);
    Complex::from(re_num / den, im_num / den)
});

impl_op_ex!(/|lhs: &Complex, rhs: &f64| -> Complex {
    Complex::from(lhs.re / rhs, lhs.im / rhs)
});

#[cfg(test)]
mod test;
