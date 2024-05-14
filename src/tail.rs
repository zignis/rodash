/// Gets all but the first element of `array`.
///
/// * `array` - The array to query.
///
/// # Examples
///
/// ```
/// use rodash::tail;
///
/// assert_eq!(tail(&[1, 2, 3]), [2, 3]);
/// ```
pub fn tail<A>(array: &[A]) -> Vec<A>
where
    A: Clone,
{
    if !array.is_empty() {
        array[1..array.len()].to_vec()
    } else {
        array.to_vec()
    }
}

/// A trait that implements the [Tail::tail] method on arrays.
pub trait Tail<A, I>
where
    A: IntoIterator,
    I: Clone,
{
    /// Gets all but the first element of this array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rodash::Tail;
    ///
    /// let array = vec![1, 2, 3];
    /// assert_eq!(array.tail(), [2, 3]);
    /// ```
    fn tail(&self) -> Vec<I>;
}

impl<A> Tail<Vec<A>, A> for Vec<A>
where
    A: Clone,
{
    fn tail(&self) -> Vec<A> {
        tail(self)
    }
}

impl<A, const N: usize> Tail<[A; N], A> for [A; N]
where
    A: Clone,
{
    fn tail(&self) -> Vec<A> {
        tail(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_exclude_the_first_element() {
        assert_eq!(tail(&[1, 2, 3]), [2, 3]);
        assert_eq!(tail(&[1, 2]), [2]);
        assert_eq!(tail(&[1]), []);

        assert_eq!(vec![1, 2, 3].tail(), [2, 3]);
        assert_eq!(vec![1, 2].tail(), [2]);
        assert_eq!(vec![1].tail(), []);

        assert_eq!([1, 2, 3].tail(), [2, 3]);
        assert_eq!([1, 2].tail(), [2]);
        assert_eq!([1].tail(), []);
    }

    #[test]
    fn can_handle_empty_arrays() {
        assert_eq!(tail::<char>(&[]), []);
        assert_eq!(
            {
                let array: Vec<char> = vec![];
                array
            }
            .tail(),
            []
        );
        assert_eq!(
            {
                let array: [char; 0] = [];
                array
            }
            .tail(),
            []
        );
    }
}
