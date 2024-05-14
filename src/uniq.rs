use std::{
    collections::HashSet,
    hash::Hash,
};

/// Creates a duplicate-free version of an array, in which only the first occurrence of each element
/// is kept. The order of result values is determined by the order they occur in the array.
///
/// * `array` - The array to inspect.
///
/// # Examples
///
/// ```
/// use rodash::uniq;
///
/// assert_eq!(uniq(&[2, 1, 2]), [2, 1]);
/// ```
pub fn uniq<A>(array: &[A]) -> Vec<A>
where
    A: Hash + Eq + Clone,
{
    let mut vec = array.to_vec();
    let mut values = HashSet::new();
    vec.retain(|e| values.insert(e.clone()));

    vec
}

/// A trait that implements the [Uniq::uniq] method on arrays.
pub trait Uniq<A, I>
where
    A: IntoIterator,
    I: Hash + Eq + Clone,
{
    /// Creates a duplicate-free version of this array, in which only the first occurrence of each
    /// element is kept. The order of result values is determined by the order they occur in the
    /// array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Uniq;
    ///
    /// assert_eq!([2, 1, 2].uniq(), [2, 1]);
    /// ```
    fn uniq(&self) -> Vec<I>;
}

impl<A> Uniq<Vec<A>, A> for Vec<A>
where
    A: Hash + Eq + Clone,
{
    fn uniq(&self) -> Vec<A> {
        uniq(self)
    }
}

impl<A, const N: usize> Uniq<[A; N], A> for [A; N]
where
    A: Hash + Eq + Clone,
{
    fn uniq(&self) -> Vec<A> {
        uniq(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_unique_values_of_an_unsorted_array() {
        assert_eq!(uniq(&[2, 1, 2]), [2, 1]);
        assert_eq!(vec![2, 1, 2].uniq(), [2, 1]);
        assert_eq!([2, 1, 2].uniq(), [2, 1]);
    }

    #[test]
    fn can_return_unique_values_of_a_sorted_array() {
        assert_eq!(uniq(&[1, 2, 2]), [1, 2]);
        assert_eq!(vec![1, 2, 2].uniq(), [1, 2]);
        assert_eq!([1, 2, 2].uniq(), [1, 2]);
    }
}
