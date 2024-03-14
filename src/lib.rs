use rug::{Float, ops::Pow};
use std::env;
use std::convert::TryFrom;

/// Calculates the value of pi to a specified number of decimal places using
/// the Gauss-Legendre algorithm.
///
/// # Arguments
///
/// * `digits` - The number of decimal places of pi to calculate.
///
/// # Returns
///
/// A `Float` representing the calculated value of pi to the specified
/// number of decimal places.
///
/// # Examples
///
/// ```
/// let pi = compute_pi(10);
/// println!("Pi to 10 decimal places: {}", pi);
/// ```
pub fn compute_pi(digits: usize) -> Float {
    let precision = ((digits as f64) * 3.3219280948874).ceil() as u32 + 10;
    let threshold = Float::with_val(precision, 10).pow(-i32::try_from(digits).unwrap());
    let mut a = Float::with_val(precision, 1);
    let two = Float::with_val(precision, 2);
    let mut b = Float::with_val(precision, 1.0 / two.sqrt());
    let mut t = Float::with_val(precision, 0.25);
    let mut p = Float::with_val(precision, 1);
    let mut pi_old = Float::with_val(precision, 0);

    let pi_result = loop {
        let sum = a.clone() + b.clone();
        let a_next = Float::with_val(precision, &sum / 2.0);
        let product = Float::with_val(precision, &a * &b);
        b = product.sqrt();
        let difference = Float::with_val(precision, &a - &a_next);
        let difference_squared = difference.pow(2);
        t -= &p * difference_squared;
        a = a_next;
        p *= 2;
        let denominator = Float::with_val(precision, &t * 4.0);
        let numerator = Float::with_val(precision, (&sum).pow(2));
        let pi = numerator / denominator;
        let pi_diff = Float::with_val(pi.prec(), &pi - &pi_old).abs();
        if pi_diff < threshold {
            break pi;
        }
        pi_old = pi;
    };
    pi_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pi_10_digits() {
        let pi_computed = compute_pi(10);
        let pi_expected = Float::with_val(256, 3.1415926535);
        assert!((pi_computed - &pi_expected).abs() < Float::with_val(256, 1e-10));
    }
}
