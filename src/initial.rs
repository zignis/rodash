/// Gets all but the last element of `array`.
///
/// * `array` - The array to query.
///
/// # Examples
///
/// ```
/// use rodash::initial;
///
/// assert_eq!(initial(&[1, 2, 3]), [1, 2]);
/// ```
pub fn initial<A>(array: &[A]) -> Vec<A>
where
    A: Clone,
{
    if !array.is_empty() {
        array[0..array.len() - 1].to_vec()
    } else {
        array.to_vec()
    }
}

/// A trait that implements the [Initial::initial] method on arrays.
pub trait Initial<A, I>
where
    A: IntoIterator,
    I: Clone,
{
    /// Gets all but the last element of this array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Initial;
    ///
    /// let array = vec![1, 2, 3];
    /// assert_eq!(array.initial(), [1, 2]);
    /// ```
    fn initial(&self) -> Vec<I>;
}

impl<A> Initial<Vec<A>, A> for Vec<A>
where
    A: Clone,
{
    fn initial(&self) -> Vec<A> {
        initial(self)
    }
}

impl<A, const N: usize> Initial<[A; N], A> for [A; N]
where
    A: Clone,
{
    fn initial(&self) -> Vec<A> {
        initial(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_exclude_the_last_element() {
        assert_eq!(initial(&[1, 2, 3]), [1, 2]);
        assert_eq!(initial(&[1, 2]), [1]);
        assert_eq!(initial(&[1]), []);

        assert_eq!(vec![1, 2, 3].initial(), [1, 2]);
        assert_eq!(vec![1, 2].initial(), [1]);
        assert_eq!(vec![1].initial(), []);

        assert_eq!([1, 2, 3].initial(), [1, 2]);
        assert_eq!([1, 2].initial(), [1]);
        assert_eq!([1].initial(), []);
    }

    #[test]
    fn can_handle_empty_arrays() {
        assert_eq!(initial::<char>(&[]), []);
        assert_eq!(
            {
                let array: Vec<char> = vec![];
                array
            }
            .initial(),
            []
        );
        assert_eq!(
            {
                let array: [char; 0] = [];
                array
            }
            .initial(),
            []
        );
    }
}
