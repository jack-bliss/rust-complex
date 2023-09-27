use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(/|lhs: &Complex, rhs: &Complex| -> Complex {
    let den = rhs.re.powi(2) + rhs.im.powi(2);
    let re_num = (lhs.re * rhs.re) + (lhs.im * rhs.im);
    let im_num = (lhs.im * rhs.re) - (lhs.re * rhs.im);
    Complex::from(re_num / den, im_num / den)
});

impl_op_ex!(/|lhs: &Complex, rhs: &f64| -> Complex {
    Complex::from(lhs.re / rhs, lhs.im / rhs)
});

impl_op_ex!(/|lhs: &f64, rhs: &Complex| -> Complex {
    let num = lhs * !rhs;
    let den = rhs * !rhs;
    num / den
});

#[cfg(test)]
mod test_divide_operator {
    use crate::Complex;

    #[test]
    fn should_divide_complex_numbers_by_each_other() {
        let a = Complex::from(4., 4.);
        let b = Complex::from(2., 2.);
        assert_eq!(&a / &b, Complex::from(2., 0.));
        assert_eq!(a / b, Complex::from(2., 0.));
    }

    #[test]
    fn should_divide_complex_number_by_float() {
        let a = Complex::from(4., 4.);
        let b = 2.;
        assert_eq!(&a / &b, Complex::from(2., 2.));
        assert_eq!(a / b, Complex::from(2., 2.));
    }

    #[test]
    fn should_divide_float_by_complex_number() {
        let a = Complex::from(4., 4.);
        let b = 2.;
        assert_eq!(&b / &a, Complex::from(0.25, -0.25));
        assert_eq!(1. / Complex::from(3., 4.), Complex::from(0.12, -0.16));
    }
}
