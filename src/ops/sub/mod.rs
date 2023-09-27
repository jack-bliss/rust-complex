use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(-|lhs: &Complex, rhs: &Complex| -> Complex {
    Complex::from(lhs.re - rhs.re, lhs.im - rhs.im)
});
impl_op_ex!(-|cmp: &Complex| -> Complex { Complex::from(-cmp.re, -cmp.im) });

#[cfg(test)]
mod test;
