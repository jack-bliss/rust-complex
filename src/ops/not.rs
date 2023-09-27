use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(!|cmp: &Complex| -> Complex { Complex::from(cmp.re, -cmp.im) });

#[cfg(test)]
mod test_not_operator {
    use crate::Complex;

    #[test]
    fn should_give_the_complex_conjugate() {
        assert_eq!(!Complex::from(1., 1.), Complex::from(1., -1.))
    }
}
