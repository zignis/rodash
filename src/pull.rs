use crate::pull_all;

/// Removes a single given value from `array`.
///
/// * `array` - The array to modify.
/// * `value` - The value to remove.
///
/// # Examples
///
/// ```
/// use rodash::pull;
///
/// let mut array = vec!['a', 'b', 'c', 'a', 'b', 'c'];
/// pull(&mut array, 'a');
/// assert_eq!(array, ['b', 'c', 'b', 'c']);
/// ```
pub fn pull<A>(array: &mut Vec<A>, value: A)
where
    A: PartialEq + Ord,
{
    pull_all(array, &[value], None);
}

/// A trait that implements the [Pull::pull] method on arrays.
pub trait Pull<A, I>
where
    A: IntoIterator,
    I: PartialEq + Ord,
{
    /// Removes a single given value from this array.
    ///
    /// * `value` - The value to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Pull;
    ///
    /// let mut array = vec!['a', 'b', 'c', 'a', 'b', 'c'];
    /// array.pull('a');
    /// assert_eq!(array, ['b', 'c', 'b', 'c']);
    /// ```
    fn pull(&mut self, value: I);
}

impl<A> Pull<Vec<A>, A> for Vec<A>
where
    A: PartialEq + Ord,
{
    fn pull(&mut self, value: A) {
        pull(self, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_pull_a_value_from_an_array() {
        let mut array = vec![1, 2, 3, 1, 3];
        pull(&mut array, 1);
        assert_eq!(array, [2, 3, 3]);

        let mut array = vec![1, 2, 3, 1, 3];
        array.pull(1);
        assert_eq!(array, [2, 3, 3]);
    }
}
