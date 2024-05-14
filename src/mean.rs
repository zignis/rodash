use num::ToPrimitive;
use std::iter::Sum;

/// Computes the mean of the values in `array`.
///
/// * `array` - The array to iterate over.
///
/// # Examples
///
/// ```
/// use rodash::mean;
///
/// assert_eq!(mean(&[4, 2, 8, 6]), 5.0);
/// ```
pub fn mean<N>(array: &[N]) -> f32
where
    N: ToPrimitive + Sum<N> + Clone,
{
    let array = array.to_vec();
    let length = array.len() as f32;
    let sum = array.into_iter().sum::<N>();

    N::to_f32(&sum).map(|sum| sum / length).unwrap_or_default()
}

/// A trait that implements the [Mean::mean] method on arrays.
pub trait Mean<A, I>
where
    A: IntoIterator,
    I: Sum + Clone + ToPrimitive,
{
    /// Computes the mean of the values in this array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Mean;
    ///
    /// assert_eq!([4, 2, 8, 6].mean(), 5.0);
    /// ```
    fn mean(&self) -> f32;
}

impl<A> Mean<Vec<A>, A> for Vec<A>
where
    A: Sum + Clone + ToPrimitive,
{
    fn mean(&self) -> f32 {
        mean(self)
    }
}

impl<A, const N: usize> Mean<[A; N], A> for [A; N]
where
    A: Sum + Clone + ToPrimitive,
{
    fn mean(&self) -> f32 {
        mean(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_compute_mean_of_an_array_of_numbers() {
        assert_eq!(mean(&[4, 2, 8, 6]), 5.0);

        assert_eq!(vec![4, 2, 8, 6].mean(), 5.0);

        assert_eq!([4, 2, 8, 6].mean(), 5.0);
    }
}
