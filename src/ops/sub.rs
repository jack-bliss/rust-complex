use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(-|lhs: &Complex, rhs: &Complex| -> Complex {
    Complex::from(lhs.re - rhs.re, lhs.im - rhs.im)
});
impl_op_ex!(-|lhs: &Complex, rhs: &f64| -> Complex { Complex::from(lhs.re - rhs, lhs.im) });
impl_op_ex!(-|lhs: &f64, rhs: &Complex| -> Complex { Complex::from(lhs - rhs.re, -rhs.im) });
impl_op_ex!(-|cmp: &Complex| -> Complex { Complex::from(-cmp.re, -cmp.im) });

#[cfg(test)]
mod test_subtract_operator {
    use crate::Complex;

    #[test]
    fn should_subtract_one_complex_number_from_another() {
        let a = Complex::from(2., 3.);
        let b = Complex::from(1., 1.);
        assert_eq!(&a - &b, Complex::from(1., 2.));
        assert_eq!(a - b, Complex::from(1., 2.));
    }

    #[test]
    fn should_negate_complex_number_when_used_as_unary_operator() {
        let a = Complex::from(2., 3.);
        assert_eq!(-a, Complex::from(-2., -3.));
    }

    #[test]
    fn should_subtract_complex_number_from_float() {
        let a = Complex::from(2., 3.);
        assert_eq!(2. - a, Complex::from(0., -3.));
    }

    #[test]
    fn should_subtract_float_from_complex_number() {
        let a = Complex::from(2., 3.);
        assert_eq!(a - 2., Complex::from(0., 3.));
    }
}
