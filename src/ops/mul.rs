use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(*|lhs: &Complex, rhs: &Complex| -> Complex {
    let re = (lhs.re * rhs.re) - (lhs.im * rhs.im);
    let im = (lhs.re * rhs.im) + (lhs.im * rhs.re);
    Complex::from(re, im)
});

impl_op_ex_commutative!(*|lhs: &Complex, rhs: &f64| -> Complex {
    Complex::from(lhs.re * rhs, lhs.im * rhs)
});

#[cfg(test)]
mod test_multiply_operator {
    use crate::Complex;

    #[test]
    fn should_multiply_two_complex_numbers_together() {
        let a = Complex::from(1., 3.);
        let b = Complex::from(1., 1.);
        assert_eq!(&a * &b, Complex::from(-2., 4.));
        assert_eq!(a * b, Complex::from(-2., 4.));
    }

    #[test]
    fn should_multiply_complex_number_by_a_float() {
        let a = Complex::from(1., 3.);
        let b = 2.;
        assert_eq!(&a * &b, Complex::from(2., 6.));
        assert_eq!(&b * &a, Complex::from(2., 6.));
        assert_eq!(a * b, Complex::from(2., 6.));
    }
}
