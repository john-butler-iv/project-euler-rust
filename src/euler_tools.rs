pub mod additional_number_contansts;
pub mod prime_finder;

use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use additional_number_contansts::MorePositiveConstants;
use integer_sqrt::IntegerSquareRoot;
use num_bigint::BigUint;
use num_traits::{CheckedAdd, One, PrimInt, Unsigned, Zero};

pub struct Fibonacci<I: Clone + Zero + One + CheckedAdd> {
    curr: Option<I>,
    prev: I,
}
impl<I: Clone + Zero + One + CheckedAdd> Iterator for Fibonacci<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let orig_curr = self.curr.clone()?;
        self.curr = orig_curr.checked_add(&self.prev);
        self.prev = orig_curr.clone();
        Some(orig_curr)
    }
}

pub fn fibonacci_iterator<I: Clone + Zero + One + CheckedAdd>() -> Fibonacci<I> {
    Fibonacci {
        curr: Some(I::zero()),
        prev: I::one(),
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

pub fn triangle<
    I: One + MorePositiveConstants + Add<I, Output = I> + Mul + Div<I, Output = I> + Clone,
>(
    n: I,
) -> I {
    // n * (n + 1) / 2
    n.clone().mul(n.clone().add(I::one())).div(I::two())
}

pub fn inverse_triange<
    I: IntegerSquareRoot
        + MorePositiveConstants
        + One
        + Add<I, Output = I>
        + Mul
        + Div<I, Output = I>
        + Sub<I, Output = I>
        + Clone,
>(
    tri: I,
) -> I {
    // (isqrt(8 * n + 1) - 1 ) / 2
    ((I::eight().mul(tri.clone()).add(I::one()))
        .integer_sqrt()
        .sub(I::one()))
    .div(I::two())
}

#[allow(dead_code)]
pub fn factorial<I: Unsigned + PrimInt>(n: I) -> Option<I> {
    let mut fact: I = I::one();
    for i in 2..=n
        .to_u128()
        .expect("n is unsigned, and u128 is the biggest primative uint")
    {
        fact = fact.checked_mul(&I::from(i).expect("must be less than n, which is I"))?
    }
    Some(fact)
}

pub fn big_factorial(n: BigUint) -> BigUint {
    let mut fact: BigUint = BigUint::one();
    let mut i = BigUint::two();
    loop {
        if i.cmp(&n) == Ordering::Greater {
            break;
        }
        fact = fact.mul(&i);
        i = i.add(BigUint::one());
    }

    fact
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::Pow;

    use crate::euler_tools::{big_factorial, factorial};

    use super::{fibonacci_iterator, Fibonacci};

    #[test]
    fn test_fib() {
        let mut it: Fibonacci<u32> = fibonacci_iterator();
        let known_fib_vals = [0, 1, 1, 2, 3, 5, 8, 13, 21];
        for known_fib in known_fib_vals.iter() {
            assert_eq!(Some(known_fib.to_owned()), it.next())
        }
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

    #[test]
    fn small_lam_w() {
        // values computed using https://www.had2know.org/academics/lambert-w-function-calculator.html
        let known_w_m1_vals = [0.0, 0.0, 0.0, -1.512135, -2.153292, -2.542641, -2.833148];

        for (u, w) in known_w_m1_vals.iter().enumerate().skip(3) {
            assert_eq!(
                w.to_string(),
                format!("{:.6}", super::lambert_w_m1_neg_inv(u as f64))
            );
        }
    }

    #[test]
    fn big_lam_w() {
        // values computed using https://www.had2know.org/academics/lambert-w-function-calculator.html
        let known_w_m1_vals = [
            0.0, 0.0, -6.472775, -9.118006, -11.667115, -14.163601, -16.626509, -19.066002,
        ];

        for (pow, w) in known_w_m1_vals.iter().enumerate().skip(2) {
            assert_eq!(
                w.to_string(),
                format!(
                    "{:.6}",
                    super::lambert_w_m1_neg_inv(10.0_f64.pow(pow as f64))
                )
            );
        }
    }

    #[test]
    fn first_ten_tri_nums() {
        const MAX_TRI: u64 = 10;
        let mut curr_tri = 0;
        for n in 0..=MAX_TRI {
            curr_tri += n;
            assert_eq!(curr_tri, super::triangle(n));
        }
    }

    #[test]
    fn u32_factorials() {
        let known_factorials: Vec<Option<u32>> = vec![
            Some(1),           // 0
            Some(1),           // 1
            Some(2),           // 2
            Some(6),           // 3
            Some(24),          // 4
            Some(120),         // 5
            Some(720),         // 6
            Some(5040),        // 7
            Some(40_320),      // 8
            Some(362_880),     // 9
            Some(3_628_800),   // 10
            Some(39_916_800),  // 11
            Some(479_001_600), // 12
            None,              // 13
        ];
        for (i, known_factorial) in known_factorials.iter().enumerate() {
            let computed_factorial = &factorial(i as u32);
            assert_eq!(computed_factorial, known_factorial, "factorial #{i}")
        }
    }
    #[test]
    fn first_22_big_factorials() {
        let known_factorials: Vec<u128> = vec![
            1,                             // 0
            1,                             // 1
            2,                             // 2
            6,                             // 3
            24,                            // 4
            120,                           // 5
            720,                           // 6
            5040,                          // 7
            40_320,                        // 8
            362_880,                       // 9
            3_628_800,                     // 10
            39_916_800,                    // 11
            479_001_600,                   // 12
            6_227_020_800,                 // 13
            87_178_291_200,                // 14
            1_307_674_368_000,             // 15
            20_922_789_888_000,            // 16
            355_687_428_096_000,           // 17
            6_402_373_705_728_000,         // 18
            121_645_100_408_832_000,       // 19
            2_432_902_008_176_640_000,     // 20
            51_090_942_171_709_440_000,    // 21
            1_124_000_727_777_607_680_000, // 22
        ]; // the on-line encyclopedia of integer sequences ends here
        for (i, known_factorial) in known_factorials.iter().enumerate() {
            let computed_factorial = &big_factorial(BigUint::from(i));
            assert_eq!(
                *computed_factorial,
                BigUint::from(*known_factorial),
                "factorial #{i}"
            )
        }
    }
}
