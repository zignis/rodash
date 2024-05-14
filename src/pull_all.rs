use std::ops::Not;

/// This method is like [pull] except that it accepts an array of values to remove.
///
/// * `array` - The array to modify.
/// * `values` - The values to remove.
/// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that `values`
///   is a sorted array. Internally, binary search is utilized when this flag is provided and is
///   truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time complexity of
///   linear search otherwise.
///
/// # Examples
///
/// ```
/// use rodash::pull_all;
///
/// let mut array = vec!['a', 'b', 'c', 'a', 'b', 'c'];
/// pull_all(&mut array, &['a', 'c'], None);
/// assert_eq!(array, ['b', 'b']);
/// ```
///
/// [pull]: crate::pull
pub fn pull_all<A>(array: &mut Vec<A>, values: &[A], is_sorted: Option<bool>)
where
    A: PartialEq + Ord,
{
    array.retain(|x| {
        if is_sorted.unwrap_or_default() {
            values.binary_search(x).is_ok()
        } else {
            values.contains(x)
        }
        .not()
    });
}

/// A trait that implements the [PullAll::pull_all] method on arrays.
pub trait PullAll<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord,
{
    /// This method is like [pull] except that it accepts an array of values to remove.
    ///
    /// * `values` - The values to remove.
    /// * `is_sorted` - The sorted flag. Should be set to `Some(true)` if you are certain that
    ///   `values` is a sorted array. Internally, binary search is utilized when this flag is
    ///   provided and is truthy, enabling searching in `O(log n)` time, compared to the `O(n)` time
    ///   complexity of linear search otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::PullAll;
    ///
    /// let mut array = vec!['a', 'b', 'c', 'a', 'b', 'c'];
    /// array.pull_all(&['a', 'c'], None);
    /// assert_eq!(array, ['b', 'b']);
    /// ```
    fn pull_all(&mut self, values: &[I], is_sorted: Option<bool>);
}

impl<A> PullAll<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord,
{
    fn pull_all(&mut self, values: &[A], is_sorted: Option<bool>) {
        pull_all(self, values, is_sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_pull_values_from_an_array() {
        let mut array = vec![1, 2, 3, 1, 3];
        pull_all(&mut array, &[1, 3], None);
        assert_eq!(array, [2]);

        let mut array = vec![1, 2, 3, 1, 3];
        array.pull_all(&[1, 3], None);
        assert_eq!(array, [2]);
    }

    #[test]
    fn can_pull_sorted_values_from_an_array() {
        let mut array = vec![1, 2, 3];
        pull_all(&mut array, &[1, 3], Some(true));
        assert_eq!(array, [2]);

        let mut array = vec![1, 2, 3];
        array.pull_all(&[1, 3], Some(true));
        assert_eq!(array, [2]);
    }
}
