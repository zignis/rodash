use crate::intersect;
use std::hash::Hash;

/// Creates an array of unique values that are included in both the `array` and `others` nested
/// arrays. The order of result values are determined by the `array`.
///
/// * `array` - The source array.
/// * `others` - The nested arrays to inspect.
///
/// # Examples
///
/// ```
/// use rodash::intersect_all;
///
/// let result = intersect_all(&[2, 1, 2, 3], vec![&[3, 4], &[3, 2]]);
/// assert_eq!(result, [3]);
/// ```
pub fn intersect_all<A>(array: &[A], others: Vec<&[A]>) -> Vec<A>
where
    A: PartialEq + Ord + Clone + Hash,
{
    let mut common = array.to_vec();

    for item in others {
        common = intersect(&common, item);
    }

    common
}

/// A trait that implements the [IntersectAll::intersect_all] method on arrays.
pub trait IntersectAll<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord + Clone + Hash,
{
    /// Creates an array of unique values that are included in both the `array` and `others` nested
    /// arrays. The order of result values are determined by the `array`.
    ///
    /// * `others` - The nested arrays to inspect.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::IntersectAll;
    ///
    /// assert_eq!([2, 1, 2, 3].intersect_all(vec![&[3, 4], &[3, 2]]), [3]);
    /// ```
    fn intersect_all(&self, others: Vec<&[I]>) -> Vec<I>;
}

impl<A> IntersectAll<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord + Clone + Hash,
{
    fn intersect_all(&self, others: Vec<&[A]>) -> Vec<A> {
        intersect_all(self, others)
    }
}

impl<A, const N: usize> IntersectAll<[A; N], A> for [A; N]
where
    A: PartialEq + Ord + Clone + Hash,
{
    fn intersect_all(&self, others: Vec<&[A]>) -> Vec<A> {
        intersect_all(self, others)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_intersection_of_multiple_arrays() {
        let result = intersect_all(&[2, 1, 2, 3], vec![&[3, 4], &[3, 2]]);
        assert_eq!(result, [3]);

        assert_eq!(vec![2, 1, 2, 3].intersect_all(vec![&[3, 4], &[3, 2]],), [3]);

        assert_eq!([2, 1, 2, 3].intersect_all(vec![&[3, 4], &[3, 2]],), [3]);
    }
}
