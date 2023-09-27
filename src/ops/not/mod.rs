use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(!|cmp: &Complex| -> Complex { Complex::from(cmp.re, -cmp.im) });

#[cfg(test)]
mod test;
