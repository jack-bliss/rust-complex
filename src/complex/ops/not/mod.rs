use impl_ops::*;
use std::ops;

use super::super::Complex;

impl_op_ex!(!|cmp: &Complex| -> Complex { Complex::from(cmp.re, -cmp.im) });

#[cfg(test)]
mod test;
