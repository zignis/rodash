use std::ops::Not;

/// Creates a vector of `array` values not included in the `other` array.
///
/// * `array` - The array to inspect.
/// * `other` - The values to exclude.
/// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that `other`
///   is a sorted array. Internally, binary search is utilized when this flag is provided and is
///   truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time complexity of
///   linear search otherwise.
///
/// # Examples
///
/// ```
/// use rodash::difference;
///
/// let result = difference(&[2, 1], &[2, 3], None);
/// assert_eq!(result, [1]);
///
/// let sorted_values = [2, 3, 4, 5];
/// let result = difference(&[2, 1], &sorted_values, Some(true));
/// assert_eq!(result, [1]);
/// ```
pub fn difference<A>(array: &[A], other: &[A], is_sorted: Option<bool>) -> Vec<A>
where
    A: PartialEq + Ord + Clone,
{
    array
        .iter()
        .filter_map(|item| {
            if is_sorted.unwrap_or_default() {
                other.binary_search(item).is_ok()
            } else {
                other.contains(item)
            }
            .not()
            .then_some(item.to_owned())
        })
        .collect()
}

/// A trait that implements the [Difference::difference] method on arrays.
pub trait Difference<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord + Clone,
{
    /// Creates a vector of values in this array that are not included in the `other` array.
    ///
    /// * `other` - The values to exclude.
    /// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that
    ///   `other` is a sorted array. Internally, binary search is utilized when this flag is
    ///   provided and is truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time
    ///   complexity of linear search otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Difference;
    ///
    /// assert_eq!([2, 1].difference(&[2, 3], None), [1]);
    ///
    /// let sorted_values = [2, 3, 4, 5];
    /// assert_eq!([2, 1].difference(&sorted_values, Some(true)), [1]);
    /// ```
    fn difference(&self, other: &[I], is_sorted: Option<bool>) -> Vec<I>;
}

impl<A> Difference<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord + Clone,
{
    fn difference(&self, other: &[A], is_sorted: Option<bool>) -> Vec<A> {
        difference(self, other, is_sorted)
    }
}

impl<A, const N: usize> Difference<[A; N], A> for [A; N]
where
    A: PartialEq + Ord + Clone,
{
    fn difference(&self, other: &[A], is_sorted: Option<bool>) -> Vec<A> {
        difference(self, other, is_sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_difference_of_two_arrays() {
        let result = difference(&[2, 1], &[2, 3], None);
        assert_eq!(result, [1]);

        assert_eq!(vec![2, 1].difference(&[2, 3], None), [1]);

        assert_eq!([2, 1].difference(&[2, 3], None), [1]);
    }

    #[test]
    fn can_return_difference_of_two_sorted_arrays() {
        let result = difference(&[2, 1], &[2, 3, 4, 5], Some(true));
        assert_eq!(result, [1]);

        assert_eq!(vec![2, 1].difference(&[2, 3, 4, 5], Some(true)), [1]);

        assert_eq!([2, 1].difference(&[2, 3, 4, 5], Some(true)), [1]);
    }
}
