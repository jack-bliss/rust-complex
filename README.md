# Complex

- WIP project for working with complex numbers in Rust

## Creating complex numbers

```rust
// creating directly
let raw = Complex {
    re: 1.,
    im: 1.,
}; // 1+1i

// shorthand (re, im)
let from = Complex::from(1., 1.); // 1+1i
let from_re = Complex::re(3.); // 3+0i
let from_im = Complex::im(3.); // 0+3i
let unit = Complex::unit(); // 1+0i
let zero = Complex::zero(); // 0+0i

// from string
let from_str = Complex::from_str("2+3i"); // 2+3i

// from polar coords
use std::f64::consts::PI;
let from_polar = Complex::from_polar(1., PI / 2.); // 0+1i

// generating roots of unity, for example, 3 roots of unity
// always includes 1+0i as the first entry
let roots: Vec<Complex> = Complex::roots_of_unity(3);
assert_eq!(roots.len(), 3);
```

## Operations

```rust
let z3 = &z1 + &z2; // addition
let z4 = &z1 - &z2; // subtraction
let z5 = &z1 * &z2; // multiplication
let z5 = &z1 * 2.; // multiplication by float
let z6 = &z1 / &z2; // division
let z6 = &z1 / 2.; // division by float
let z7 = -&z1; // negation
let z8 = !&z1; // complex conjugation (a+bi -> a-bi)

let z9 = &z1.pow(2.) // power
let z9 = &z1.powi(2) // integer power (doesn't use trig)

let abs = &z1.abs(); // absolute value (magnitude)
let arg = &z1.arg(); // argument (angle from real line)
let PolarForm {abs, arg} = &z1.to_polar();
let stringified = z1.to_str(); // convert to string representation
```
