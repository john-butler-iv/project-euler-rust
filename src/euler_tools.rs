pub mod additional_number_constants;
pub mod collection_tools;
pub mod figurate_numbers;
pub mod prime_finder;
pub mod ratio;

use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};

use additional_number_constants::MorePositiveConstants;
use num_bigint::BigUint;
use num_traits::{CheckedAdd, CheckedMul, ConstOne, Num, One, PrimInt, Zero};
use prime_finder::CoprimePairsIterator;

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

impl<I> DigitIterator<I>
where
    DigitIterator<I>: Iterator,
{
    pub fn new_radix(number: I, radix: I) -> Self {
        DigitIterator {
            remaining_number: number,
            radix,
        }
    }
}
pub struct DigitIterator<I> {
    remaining_number: I,
    radix: I,
}

macro_rules! digit_iterator_impl {
    ($($prim_type:ty),*) => { $(
        impl Iterator for DigitIterator<$prim_type>{
            type Item = $prim_type;

            fn next(&mut self) -> Option<Self::Item>{
                if self.remaining_number == 0{
                    None
                } else {
                    let digit = self.remaining_number % self.radix;
                    self.remaining_number /= self.radix;
                    Some(digit)
                }
            }
        }
        impl DigitIterator<$prim_type>{
            #[allow(dead_code)]
            pub fn new(number: $prim_type) -> Self {
                DigitIterator::new_radix(number, 10)
            }
            #[allow(dead_code)]
            pub fn combine_digits(digits: &[u8]) -> $prim_type{
                let mut num:$prim_type = 0;
                for digit in digits.iter().rev(){
                    num = 10 * num + (*digit as $prim_type);
                }
                num
            }

            #[allow(dead_code)]
            pub fn combine_digits_big_endian(digits:&[$prim_type]) ->$prim_type {
                let mut num = 0;
                for digit in digits.iter(){
                    num = 10 * num + digit;
                }
                num
            }

            #[allow(dead_code)]
            pub fn combine_digits_rotated(digits: &[$prim_type], starting_index: usize) -> $prim_type {
                let mut num = 0;
                for i in (starting_index..starting_index+ digits.len() ).rev(){
                    let digit = digits[i % digits.len()];
                    num = 10 * num + digit;
                }
                num
            }

            #[allow(dead_code)]
            pub fn are_digital_permutations(digit_set1: &[u8; 10], digit_set2: &[u8; 10])-> bool{
                for (digit_count1, digit_count2) in digit_set1.iter().zip(digit_set2.iter()) {
                    if digit_count1 != digit_count2 {
                        return false;
                    }
                }
                true
            }

            #[allow(dead_code)]
            pub fn create_digit_set(num: $prim_type) -> [u8; 10]{
                let mut digit_set = [0;10];
                for digit in DigitIterator::<$prim_type>::new(num){
                    digit_set[digit as usize] += 1;
                }

                digit_set
            }
        }
    )* };
}

digit_iterator_impl!(u8, u16, u32, u64, u128, usize);
digit_iterator_impl!(i8, i16, i32, i64, i128, isize);

pub trait RotateDigits: Sized {
    fn rotate_digits(n: Self, len: u32) -> Self;
    #[allow(dead_code)]
    fn rotate_digits_unsized(n: Self) -> Self;
}
macro_rules! rotate_digits_impl {
    ($($prim_type:ty),*) => { $(
        impl RotateDigits for $prim_type {
            fn rotate_digits(n: Self, len: u32)-> $prim_type{
                let mut n = n;
                let digit = n % 10;
                n /= 10;
                n+digit*10.pow(len - 1)
            }
             fn rotate_digits_unsized(n: Self)-> $prim_type{
                let mut len = 0;
                {
                    let mut n = n;
                    while n != 0 {
                        n /= 10;
                        len += 1;
                    }
                };

                RotateDigits::rotate_digits(n, len)
            }
        }
    )* };
}
rotate_digits_impl!(u8, u16, u32, u64, u128, usize);
rotate_digits_impl!(i8, i16, i32, i64, i128, isize);

pub trait IsPandigital: Sized {
    #[allow(dead_code)]
    fn is_limited_pandigital(&self) -> bool;
    #[allow(dead_code)]
    fn is_pandigital(&self) -> bool;
    fn are_combined_pandigital(components: &[Self]) -> bool;
}
macro_rules! is_pandigital_impl {
    ($($prim_type:ty),*) => { $(
        impl IsPandigital for $prim_type {
            fn is_limited_pandigital(&self) -> bool{
                let mut digit_set = [false;10];
                digit_set[0] = true;
                let mut total_digits = 0;
                let mut max_digit = 0;
                for digit in DigitIterator::<$prim_type>::new(*self) {
                    if digit_set[digit as usize] {return false;}
                    digit_set[digit as usize] = true;
                    total_digits += 1;
                    max_digit = std::cmp::max(max_digit, digit)
                }
                return max_digit == total_digits;
            }
            fn is_pandigital(&self)-> bool{
                let mut digit_set = [false;10];
                digit_set[0] = true;
                let mut total_digits = 0;
                for digit in DigitIterator::<$prim_type>::new(*self) {
                    if digit_set[digit as usize] {return false;}
                    digit_set[digit as usize] = true;
                    total_digits += 1;
                }
                total_digits == 9
            }
            fn are_combined_pandigital(components: &[Self]) -> bool {
                let mut digit_set = [false;10];
                digit_set[0] = true;
                let mut total_digits = 0;
                for n in components{
                    for digit in DigitIterator::<$prim_type>::new(*n) {
                        if digit_set[digit as usize] {return false;}
                        digit_set[digit as usize] = true;
                        total_digits += 1;
                    }
                }
                total_digits == 9
            }
        }
    )* }
}
is_pandigital_impl!(u8, u16, u32, u64, u128, usize);
is_pandigital_impl!(i8, i16, i32, i64, i128, isize);

pub fn is_bin_palindrome(n: usize) -> bool {
    let mut constructor = n;
    let mut rev_n = 0;
    while constructor != 0 {
        rev_n = (rev_n << 1) | (constructor & 1);
        constructor >>= 1;
    }
    rev_n == n
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

#[allow(dead_code)]
pub fn factorial<I: PrimInt + ConstOne>(n: I) -> Option<I> {
    let mut fact: I = I::ONE;
    for i in 2..=n
        .to_u128()
        .expect("n is unsigned, and u128 is the biggest primative uint")
    {
        fact = fact.checked_mul(&I::from(i).expect("must be less than n, which is I"))?
    }
    Some(fact)
}

pub fn sized_factorial_table<I: Num + CheckedMul + One + TryFrom<usize>>(size: usize) -> Vec<I> where
{
    let mut table = Vec::with_capacity(size);
    table.push(I::one());

    for n in 1..size {
        if let Ok(i) = I::try_from(n) {
            if let Some(factorial) = table[n - 1].checked_mul(&i) {
                table.push(factorial);
            } else {
                panic!("largest factorial could not fit in type");
            }
        } else {
            panic!("largest factorial could not fit in type");
        }
    }

    table
}

pub fn bounded_factorial_table<I: Num + CheckedMul + Add + One + Ord + Clone>(limit: I) -> Vec<I> {
    let mut table = Vec::new();

    let mut fact: I = I::one();
    let mut n = I::zero();
    while fact.cmp(&limit) == Ordering::Less {
        table.push(fact.clone());

        n = n.add(I::one());
        fact = match fact.checked_mul(&n) {
            None => return table,
            Some(new_fact) => new_fact,
        };
    }

    table
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

#[derive(Debug)]
pub struct PythagoreanTripleGenerator {
    core_gen: PrimPythagoreanTripleGenerator,
    limit: u64,
    next_prim_triple: Option<(u64, u64, u64)>,
    next_scale: u64,
}

#[derive(Debug)]
pub struct PrimPythagoreanTripleGenerator {
    coprime_gen: CoprimePairsIterator,
    limit: u64,
}

impl Iterator for PythagoreanTripleGenerator {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_triple = self.next_prim_triple?;

        if current_triple.2 * self.next_scale >= self.limit {
            self.next_prim_triple = self.core_gen.next();
            current_triple = self.next_prim_triple?;

            self.next_scale = 2;
        } else {
            current_triple.0 *= self.next_scale;
            current_triple.1 *= self.next_scale;
            current_triple.2 *= self.next_scale;

            self.next_scale += 1;
        }

        Some(current_triple)
    }
}
impl Iterator for PrimPythagoreanTripleGenerator {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut coprimes;
        let mut c;

        loop {
            coprimes = self.coprime_gen.next()?;
            c = coprimes.1 * coprimes.1 + coprimes.0 * coprimes.0;

            if c < self.limit && coprimes.0 % 2 != coprimes.1 % 2 {
                break;
            }
        }

        let a = coprimes.1 * coprimes.1 - coprimes.0 * coprimes.0;
        let b = 2 * coprimes.1 * coprimes.0;

        if a < b {
            Some((a, b, c))
        } else {
            Some((b, a, c))
        }
    }
}

impl PythagoreanTripleGenerator {
    #[allow(dead_code)]
    pub fn new(limit: u64) -> Self {
        let mut core_gen = PrimPythagoreanTripleGenerator::new(limit);
        let first_prim_triple = core_gen.next();
        PythagoreanTripleGenerator {
            core_gen,
            limit,
            next_prim_triple: first_prim_triple,
            next_scale: 1,
        }
    }

    pub fn next_prim(&mut self) {
        self.next_prim_triple = self.core_gen.next();
        self.next_scale = 1;
    }
}
impl PrimPythagoreanTripleGenerator {
    pub fn new(limit: u64) -> Self {
        PrimPythagoreanTripleGenerator {
            coprime_gen: CoprimePairsIterator::new(integer_sqrt::IntegerSquareRoot::integer_sqrt(
                &(limit * 2),
            )),
            limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::Pow;
    use std::ops::Add;

    use super::{fibonacci_iterator, Fibonacci};
    use crate::euler_tools::is_bin_palindrome;
    #[allow(unused_imports)]
    use crate::euler_tools::{
        additional_number_constants::MorePositiveConstants, big_factorial, factorial, IsPandigital,
        RotateDigits,
    };

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
    fn test_digits() {
        assert_eq!(
            super::DigitIterator::<isize>::new(12345isize).collect::<Vec<isize>>(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(
            super::DigitIterator::<isize>::new(12045isize).collect::<Vec<isize>>(),
            vec![5, 4, 0, 2, 1]
        );
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

    fn known_u32_factorials() -> Vec<u32> {
        vec![
            1,           // 0
            1,           // 1
            2,           // 2
            6,           // 3
            24,          // 4
            120,         // 5
            720,         // 6
            5040,        // 7
            40_320,      // 8
            362_880,     // 9
            3_628_800,   // 10
            39_916_800,  // 11
            479_001_600, // 12
        ]
    }

    #[test]
    fn u32_factorials() {
        let known_factorials: Vec<Option<u32>> =
            known_u32_factorials().iter().map(|n| Some(*n)).collect();
        for (i, known_factorial) in known_factorials.iter().enumerate() {
            let computed_factorial = &factorial(i as u32);
            assert_eq!(computed_factorial, known_factorial, "factorial #{i}")
        }
    }

    #[test]
    fn u32_factorial_table() {
        let known_factorials: Vec<u32> = known_u32_factorials();
        assert_eq!(known_factorials, super::bounded_factorial_table(u32::MAX));
    }

    fn known_big_factorials() -> Vec<BigUint> {
        vec![
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
        ] // the on-line encyclopedia of integer sequences ends here
        .iter()
        .map(|n: &u128| BigUint::from(*n))
        .collect()
    }

    #[test]
    fn first_22_big_factorials() {
        let known_factorials = known_big_factorials();
        for (i, known_factorial) in known_factorials.iter().enumerate() {
            let computed_factorial = big_factorial(BigUint::from(i));
            assert_eq!(computed_factorial, *known_factorial, "factorial #{i}")
        }
    }

    #[test]
    fn first_22_big_factorial_table() {
        let known_factorials = known_big_factorials();
        assert_eq!(
            known_factorials,
            super::bounded_factorial_table(
                known_factorials[known_factorials.len() - 1]
                    .clone()
                    .add(BigUint::two())
            )
        )
    }

    #[test]
    fn rotate_digits() {
        assert_eq!(RotateDigits::rotate_digits(12345, 5), 51234);
        assert_eq!(RotateDigits::rotate_digits_unsized(12345), 51234);
    }

    #[test]
    fn test_is_bin_palindrome() {
        assert!(is_bin_palindrome(0b10101010101));
        assert!(!is_bin_palindrome(0b1010101010));
    }

    #[test]
    fn is_pandigital() {
        assert!(192384576.is_pandigital());
        assert!(918273645.is_pandigital());

        assert!(!12345.is_pandigital());

        assert!(!11_23456789.is_pandigital());
    }

    #[test]
    fn are_pandigital() {
        assert!(i64::are_combined_pandigital(&[192, 384, 576]));
        assert!(i64::are_combined_pandigital(&[9, 18, 27, 36, 45]));
    }

    #[test]
    fn prim_pythag_triples() {
        let all_generated_prim_triples: Vec<_> =
            super::PrimPythagoreanTripleGenerator::new(50).collect();
        let all_prim_triples = [
            (3, 4, 5),
            (5, 12, 13),
            (7, 24, 25),
            (8, 15, 17),
            (9, 40, 41),
            (12, 35, 37),
            (20, 21, 29),
        ];

        assert_eq!(all_prim_triples.len(), all_generated_prim_triples.len());
        for prim_triple in all_prim_triples.iter() {
            assert!(dbg!(all_generated_prim_triples.contains(dbg!(prim_triple))));
        }
        for prim_triple in all_generated_prim_triples.iter() {
            assert!(all_prim_triples.contains(prim_triple));
        }
    }

    #[test]
    fn all_pythag_triples() {
        let all_generated_triples: Vec<_> = super::PythagoreanTripleGenerator::new(50).collect();
        let all_triples = [
            (3, 4, 5),
            (6, 8, 10),
            (9, 12, 15),
            (12, 16, 20),
            (15, 20, 25),
            (18, 24, 30),
            (21, 28, 35),
            (24, 32, 40),
            (27, 36, 45),
            (5, 12, 13),
            (10, 24, 26),
            (15, 36, 39),
            (7, 24, 25),
            (8, 15, 17),
            (16, 30, 34),
            (9, 40, 41),
            (12, 35, 37),
            (20, 21, 29),
        ];

        assert_eq!(all_triples.len(), all_generated_triples.len());
        for triple in all_triples.iter() {
            assert!(all_generated_triples.contains(triple));
        }
        for triple in all_generated_triples.iter() {
            assert!(all_triples.contains(triple));
        }
    }
}
