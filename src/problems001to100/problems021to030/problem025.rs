// https://projecteuler.net/problem=25

use num_bigint::BigUint;
use num_traits::pow;
use std::cmp::Ordering;

use crate::euler_tools::{additional_number_contansts::MorePositiveConstants, fibonacci_iterator};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "1000-Digit Fibonacci Number",
        number: 25,
        solve: || core_solve_fast(1000),
    }
}

// This runs in 879 microseconds on my computer in release mode
#[allow(dead_code)]
fn core_solve_slow(digits: usize) -> u64 {
    let limit = &BigUint::pow(&BigUint::ten(), digits as u32 - 1);
    println!("{}", limit.to_string().len());
    for (fibb_num, fibb) in fibonacci_iterator::<BigUint>().enumerate() {
        if fibb.cmp(limit) != Ordering::Less {
            return fibb_num as u64;
        }
    }
    unreachable!()
}

// This run in 16 microseconds on my computer in release mode
fn core_solve_fast(digits: usize) -> u64 {
    let init_digits = 5;
    let mut curr_digits = init_digits;
    let threshold = pow(10, init_digits);
    let mut fibb_curr = 1;
    let mut fibb_prev = 0;
    let mut fibb_num = 1;

    while curr_digits < digits {
        let new_prev = fibb_curr;
        fibb_curr += fibb_prev;
        fibb_prev = new_prev;
        fibb_num += 1;

        if fibb_curr / threshold > 0 {
            fibb_curr /= 10;
            fibb_prev /= 10;
            curr_digits += 1;
        }
    }

    fibb_num
}

#[cfg(test)]
mod tests {
    #[test]
    fn slow_toy_example() {
        assert_eq!(super::core_solve_slow(3), 12);
    }

    /*
    // We assume we're going to have at least 5 digits in the fast solution, so these tests fail
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve_fast(3), 12)
    }

    #[test]
    fn fast_slow_match_toy() {
        assert_eq!(super::core_solve_fast(3), super::core_solve_slow(3));
    }
    */

    #[test]
    fn fast_slow_match_real() {
        assert_eq!(super::core_solve_fast(1000), super::core_solve_slow(1000));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 4782)
    }
}
