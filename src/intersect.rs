use std::{
    collections::HashSet,
    hash::Hash,
};

/// Creates an array of unique values that are included in both the `array` and `other` arrays. The
/// order of result values are determined by the `array`.
///
/// * `array` - The source array.
/// * `other` - The array to inspect.
///
/// # Examples
///
/// ```
/// use rodash::intersect;
///
/// let result = intersect(&[2, 1], &[2, 3]);
/// assert_eq!(result, [2]);
/// ```
pub fn intersect<A>(array: &[A], other: &[A]) -> Vec<A>
where
    A: PartialEq + Ord + Clone + Hash,
{
    let unique_set: HashSet<A> = other.iter().map(|x| x.to_owned()).collect();

    unique_set
        .intersection(&array.iter().map(|x| x.to_owned()).collect())
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
}

/// A trait that implements the [Intersect::Intersect] method on arrays.
pub trait Intersect<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord + Clone + Hash,
{
    /// Creates an array of unique values that are included in this array and the `other` array. The
    /// order of result values are determined by this array.
    ///
    /// * `other` - The array to inspect.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Intersect;
    ///
    /// assert_eq!([2, 1].intersect(&[2, 3]), [2]);
    /// ```
    fn intersect(&self, other: &[I]) -> Vec<I>;
}

impl<A> Intersect<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord + Clone + Hash,
{
    fn intersect(&self, other: &[A]) -> Vec<A> {
        intersect(self, other)
    }
}

impl<A, const N: usize> Intersect<[A; N], A> for [A; N]
where
    A: PartialEq + Ord + Clone + Hash,
{
    fn intersect(&self, other: &[A]) -> Vec<A> {
        intersect(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_intersection_of_two_arrays() {
        let result = intersect(&[2, 1], &[2, 3]);
        assert_eq!(result, [2]);

        assert_eq!(vec![2, 1].intersect(&[2, 3],), [2]);

        assert_eq!([2, 1].intersect(&[2, 3],), [2]);
    }
}
