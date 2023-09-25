use super::Complex;

pub fn close(a: &Complex, b: &Complex) {
    assert!((a - b).abs() < 1e-10, "{} was not close to {}", a, b);
}
