mod complex;
use complex::Complex;

fn main() {
    let z1 = Complex::from(1., 1.);
    let z2 = Complex::from(3., 4.);
    z1.print_abs();
    z2.print_abs();

    println!("{0:?}, {0}", z1);

    let z4 = &z1 + z2;
    let z3 = z1 + &z4;

    let z5 = &z4 - &z3;

    let z6 = -&z5;

    let z7 = Complex::from(2., 4.) / Complex::from(2., 0.);

    z3.print_abs();
    z4.print_abs();
    z5.print_abs();
    z6.print_abs();

    z7.print_abs();

    let z8 = Complex::from_polar(3., 0.);
    z8.print_abs();
    let z9 = Complex::from_polar(2., std::f64::consts::PI / 4.);
    z9.print_abs();
}
