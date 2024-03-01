pub mod prime_finder;

use num_traits::PrimInt;

pub struct Fibonacci<T: PrimInt> {
    curr: Option<T>,
    prev: T,
}
impl<T: PrimInt> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let orig_curr = self.curr?;
        self.curr = orig_curr.checked_add(&self.prev);
        self.prev = orig_curr;
        Some(orig_curr)
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
    bound: T,
    iter: Box<dyn Iterator<Item = T>>,
}

pub struct BoundedRefIterator<'a, T>
where
    T: std::cmp::PartialOrd<T>,
{
    /// exclusive bound
    bound: T,
    iter: Box<dyn Iterator<Item = &'a T> + 'a>,
}

impl<T: PartialOrd<T>> BoundedIterator<T> {
    pub fn new(bound: T, iter: impl Iterator<Item = T> + 'static) -> Self {
        BoundedIterator {
            bound,
            iter: Box::new(iter),
        }
    }
}
impl<'a, T: PartialOrd<T>> BoundedRefIterator<'a, T> {
    pub fn new(bound: T, iter: impl Iterator<Item = &'a T> + 'a) -> Self {
        BoundedRefIterator {
            bound,
            iter: Box::new(iter),
        }
    }
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

impl<'a, T> Iterator for BoundedRefIterator<'a, T>
where
    T: std::cmp::PartialOrd<T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        if next.ge(&self.bound) {
            None
        } else {
            Some(next)
        }
    }
}

pub fn is_palindrome(string: &str) -> bool {
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

/// computes W_{-1} (-1/u)
/// implementation of approximation from R. Iacono and J.P. Boyd mentioned on https://en.wikipedia.org/wiki/Lambert_W_function
pub fn lambert_w_m1_neg_inv(u: f64) -> f64 {
    let mut w = if u >= 4.0 {
        -u.ln() - u.ln().ln()
    } else if u >= 0.0 {
        -1.0 - (2.0 * (1.0 - std::f64::consts::E / u)).sqrt()
    } else {
        panic!()
    };

    const MAX_ITERS: i32 = 64;
    for _ in 1..=MAX_ITERS {
        let init_w = w;
        w = w * (1.0 + (-1.0 / (u * w)).ln()) / (1.0 + w);
        if init_w == w {
            return w;
        }
    }

    w
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
        let it = super::BoundedIterator::new(BOUND, 0..=(BOUND + 1));

        for value in it {
            assert!(value < BOUND);
        }
    }
    #[test]
    fn test_bounded_ref_it() {
        const BOUND: i32 = 5;
        let values: Vec<i32> = (0..=(BOUND + 1)).collect();
        let it = super::BoundedRefIterator::new(BOUND, values.iter());

        for value in it {
            assert!(value < &BOUND);
        }
    }
}
