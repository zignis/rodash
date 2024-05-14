use crate::difference;

/// Creates a vector of `array` values not included in the `others` nested arrays.
///
/// * `array` - The array to inspect.
/// * `others` - The nested values to exclude.
/// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that `other`
///   is a sorted array. Internally, binary search is utilized when this flag is provided and is
///   truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time complexity of
///   linear search otherwise.
///
/// # Examples
///
/// ```
/// use rodash::difference_all;
///
/// let result = difference_all(&[2, 1, 2, 3], vec![&[3, 4], &[3, 2]], None);
/// assert_eq!(result, [1]);
/// ```
pub fn difference_all<A>(array: &[A], others: Vec<&[A]>, is_sorted: Option<bool>) -> Vec<A>
where
    A: PartialEq + Ord + Clone,
{
    let flattened = others
        .into_iter()
        .flatten()
        .map(|item| item.to_owned())
        .collect::<Vec<_>>();

    difference(array, &flattened, is_sorted)
}

/// A trait that implements the [DifferenceAll::difference_all] method on arrays.
pub trait DifferenceAll<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord + Clone,
{
    /// Creates a vector of values in this array that are not included in the `others` nested
    /// arrays.
    ///
    /// * `others` - The nested values to exclude.
    /// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that
    ///   `other` is a sorted array. Internally, binary search is utilized when this flag is
    ///   provided and is truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time
    ///   complexity of linear search otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::DifferenceAll;
    ///
    /// assert_eq!(
    ///     [2, 1, 2, 3].difference_all(vec![&[3, 4], &[3, 2]], None),
    ///     [1]
    /// );
    /// ```
    fn difference_all(&self, others: Vec<&[I]>, is_sorted: Option<bool>) -> Vec<I>;
}

impl<A> DifferenceAll<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord + Clone,
{
    fn difference_all(&self, others: Vec<&[A]>, is_sorted: Option<bool>) -> Vec<A> {
        difference_all(self, others, is_sorted)
    }
}

impl<A, const N: usize> DifferenceAll<[A; N], A> for [A; N]
where
    A: PartialEq + Ord + Clone,
{
    fn difference_all(&self, others: Vec<&[A]>, is_sorted: Option<bool>) -> Vec<A> {
        difference_all(self, others, is_sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_difference_of_multiple_arrays() {
        let result = difference_all(&[2, 1, 2, 3], vec![&[3, 4], &[3, 2]], None);
        assert_eq!(result, [1]);

        assert_eq!(
            vec![2, 1, 2, 3].difference_all(vec![&[3, 4], &[3, 2]], None),
            [1]
        );

        assert_eq!(
            [2, 1, 2, 3].difference_all(vec![&[3, 4], &[3, 2]], None),
            [1]
        );
    }

    #[test]
    fn can_return_difference_of_multiple_sorted_arrays() {
        let result = difference_all(&[2, 1, 2, 3], vec![&[3, 4, 5], &[2, 3, 4]], Some(true));
        assert_eq!(result, [1]);

        assert_eq!(
            vec![2, 1, 2, 3].difference_all(vec![&[3, 4, 5], &[2, 3, 4]], Some(true)),
            [1]
        );

        assert_eq!(
            [2, 1, 2, 3].difference_all(vec![&[3, 4, 5], &[2, 3, 4]], Some(true)),
            [1]
        );
    }
}
