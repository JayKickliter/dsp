use std::ops::{Add, Mul};
use num::Zero;

/// Returns the dot-product of two slices.
///
/// # Panics
/// Panics if `xs.len() != ys.len()`.
///
/// # Examples
/// ```
/// # use dsp::math::dotproduct::dot;
/// let xs = [1,2];
/// let ys = [1,2];
/// assert_eq!(5, dot(&xs, &ys));
/// ```
pub fn dot<T>(xs: &[T], ys: &[T]) -> T
    where T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T>
{
    assert_eq!(xs.len(), ys.len());

    let mut sum = T::zero();
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        sum = sum + x * y;
    }
    sum
}

#[cfg(test)]
mod tests {
    use ::math::dotproduct::dot;

    #[test]
    #[should_panic]
    fn test_dotproduct_unequal_len_panics() {
        let xs = [1; 1];
        let ys = [1; 2];
        dot(&xs, &ys);
    }
}
