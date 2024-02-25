pub mod prime_finder;

use num_traits::PrimInt;

pub struct Fibonacci<T: PrimInt> {
    curr: Option<T>,
    prev: T,
}
impl<T: PrimInt> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Some(curr) => {
                self.curr = curr.checked_add(&self.prev);
                self.prev = curr;
                Some(curr)
            }
            None => None,
        }
    }
}

pub fn fibonacci_iterator<T: PrimInt>() -> Fibonacci<T> {
    Fibonacci {
        curr: Some(T::zero()),
        prev: T::one(),
    }
}

pub struct BoundedIterator<T>
where
    T: std::cmp::PartialOrd<T>,
{
    /// exclusive bound
    pub bound: T,
    pub iter: Box<dyn Iterator<Item = T>>,
}

impl<T> Iterator for BoundedIterator<T>
where
    T: std::cmp::PartialOrd<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        if next.ge(&self.bound) {
            None
        } else {
            Some(next)
        }
    }
}

pub fn is_palindrom(string: &str) -> bool {
    let num_bytes = string.bytes().len();
    let mut reverse_string = string.bytes().rev();
    for (i, digit) in string.bytes().enumerate() {
        if i >= num_bytes / 2 {
            break;
        }
        if reverse_string.next().expect(
            "iterating over the same string, so we should run out of bytes at the same time",
        ) != digit
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::fibonacci_iterator;

    #[test]
    fn test_fib() {
        let mut it = fibonacci_iterator().enumerate();
        assert!(Some((0, 0)) == it.next());
        assert!(Some((1, 1)) == it.next());
        assert!(Some((2, 1)) == it.next());
        assert!(Some((3, 2)) == it.next());
        assert!(Some((4, 3)) == it.next());
        assert!(Some((5, 5)) == it.next());
        assert!(Some((6, 8)) == it.next());
        assert!(Some((7, 13)) == it.next());
        assert!(Some((8, 21)) == it.next());
    }

    #[test]
    fn test_bounded_it() {
        const BOUND: i32 = 5;
        let it = super::BoundedIterator {
            bound: BOUND,
            iter: Box::new(0..(BOUND + 1)),
        };

        for value in it {
            assert!(value < BOUND);
        }
    }
}
