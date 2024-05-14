/// Gets the index at which the first occurrence of `element` is found in the `array`.
///
/// * `array` - The array to inspect.
/// * `element` - The element to search for.
///
/// # Examples
///
/// ```
/// use rodash::index_of;
///
/// assert_eq!(index_of(&[1, 2, 3], &2), Some(1));
/// ```
pub fn index_of<A>(array: &[A], element: &A) -> Option<usize>
where
    A: PartialEq,
{
    array.iter().position(|x| x == element)
}

/// A trait that implements the [IndexOf::index_of] method on arrays.
pub trait IndexOf<A, I>
where
    A: IntoIterator,
    I: PartialEq,
{
    /// Gets the index at which the first occurrence of `element` is found in this array.
    ///
    /// * `element` - The element to search for.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::IndexOf;
    ///
    /// assert_eq!([1, 2, 3].index_of(&2), Some(1));
    /// ```
    fn index_of(&self, element: &I) -> Option<usize>;
}

impl<A> IndexOf<Vec<A>, A> for Vec<A>
where
    A: PartialEq,
{
    fn index_of(&self, element: &A) -> Option<usize> {
        index_of(self, element)
    }
}

impl<A, const N: usize> IndexOf<[A; N], A> for [A; N]
where
    A: PartialEq,
{
    fn index_of(&self, element: &A) -> Option<usize> {
        index_of(self, element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_the_index_of_first_matched_element() {
        assert_eq!(index_of(&[1, 2, 3, 1, 2, 3], &3), Some(2));
        assert_eq!(vec![1, 2, 3, 1, 2, 3].index_of(&3), Some(2));
        assert_eq!([1, 2, 3, 1, 2, 3].index_of(&3), Some(2));
    }

    #[test]
    fn can_handle_unmatched_elements() {
        assert_eq!(index_of(&[1, 2, 3, 1, 2, 3], &10), None);
        assert_eq!(vec![1, 2, 3, 1, 2, 3].index_of(&10), None);
        assert_eq!([1, 2, 3, 1, 2, 3].index_of(&10), None);
    }
}
