use rand::{
    seq::IndexedRandom,
    thread_rng,
};

/// Gets a random element from `array`.
///
/// * `array` - The array to sample.
///
/// # Examples
///
/// ```
/// use rodash::sample;
///
/// assert!(sample(&[1, 2, 3, 4]).is_some());
/// ```
pub fn sample<A>(array: &[A]) -> Option<&A> {
    array.choose(&mut thread_rng())
}

/// A trait that implements the [Sample::sample] method on arrays.
pub trait Sample<A, I>
where
    A: IntoIterator,
{
    /// Gets a random element from this array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Sample;
    ///
    /// let array = vec![1, 2, 3, 4];
    /// assert!(array.sample().is_some());
    /// ```
    fn sample(&self) -> Option<&I>;
}

impl<A> Sample<Vec<A>, A> for Vec<A> {
    fn sample(&self) -> Option<&A> {
        sample(self)
    }
}

impl<A, const N: usize> Sample<[A; N], A> for [A; N] {
    fn sample(&self) -> Option<&A> {
        sample(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_a_random_element() {
        let array = vec![1, 2, 3];
        assert!(array.contains(sample(&array).unwrap()));

        assert!(array.contains(array.sample().unwrap()));

        assert!([1, 2, 3].contains([1, 2, 3].sample().unwrap()));
    }

    #[test]
    fn can_handle_empty_arrays() {
        assert_eq!(sample::<char>(&[]), None);
        assert_eq!(
            {
                let array: Vec<char> = vec![];
                array
            }
            .sample(),
            None
        );
        assert_eq!(
            {
                let array: [char; 0] = [];
                array
            }
            .sample(),
            None
        );
    }
}
