use rand::thread_rng;

/// Creates an array of shuffled values.
///
/// * `array` - The array to shuffle.
///
/// # Examples
///
/// ```
/// use rodash::shuffle;
///
/// assert_eq!(shuffle(&[1, 2, 3, 4]).len(), 4);
/// ```
pub fn shuffle<A>(array: &[A]) -> Vec<A>
where
    A: Clone,
{
    let mut array = array.to_vec();
    let array = array.as_mut_slice();
    rand::seq::SliceRandom::shuffle(array, &mut thread_rng());
    array.to_vec()
}

/// A trait that implements the [Shuffle::shuffle] method on arrays.
pub trait Shuffle<A, I>
where
    A: IntoIterator,
    I: Clone,
{
    /// Creates an array of shuffled values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Shuffle;
    ///
    /// let array = vec![1, 2, 3, 4];
    /// assert_eq!(array.shuffle().len(), 4);
    /// ```
    fn shuffle(&self) -> Vec<I>;
}

impl<A> Shuffle<Vec<A>, A> for Vec<A>
where
    A: Clone,
{
    fn shuffle(&self) -> Vec<A> {
        shuffle(self)
    }
}

impl<A, const N: usize> Shuffle<[A; N], A> for [A; N]
where
    A: Clone,
{
    fn shuffle(&self) -> Vec<A> {
        shuffle(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_shuffle_an_array() {
        assert_eq!(shuffle(&[1, 2, 3, 4]).len(), 4);

        assert_eq!(vec![1, 2, 3, 4].shuffle().len(), 4);

        assert_eq!([1, 2, 3, 4].shuffle().len(), 4);
    }
}
