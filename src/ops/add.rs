use impl_ops::*;
use std::ops;

use crate::Complex;

impl_op_ex!(+ |lhs: &Complex, rhs: &Complex| -> Complex {
    Complex::from(lhs.re + rhs.re, lhs.im + rhs.im)
});

impl_op_ex_commutative!(+ |lhs: &Complex, rhs: &f64| -> Complex {
    Complex::from(lhs.re + rhs, lhs.im)
});

#[cfg(test)]
mod test_add_operator {
    use crate::Complex;

    #[test]
    fn should_add_together_two_complex_numbers() {
        let a = Complex::from(1., 1.);
        let b = Complex::from(2., 3.);
        assert_eq!(&a + &b, Complex::from(3., 4.));
        assert_eq!(a + b, Complex::from(3., 4.));
    }

    #[test]
    fn should_add_a_complex_number_to_a_float() {
        let a = Complex::from(2., 1.);
        let b = 2.;
        assert_eq!(&a + &b, Complex::from(4., 1.));
        assert_eq!(b + a, Complex::from(4., 1.));
    }
}
