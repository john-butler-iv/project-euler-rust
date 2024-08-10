// https://projecteuler.net/problem=48

use std::ops::Add;

use num_bigint::BigUint;
use num_traits::One;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Self Powers",
        number: 48,
        solve: || core_solve(1000),
    }
}

fn core_solve(last_num: u32) -> i64 {
    let mut sum = BigUint::ZERO;

    let one = BigUint::one();
    let mut big_n = BigUint::one();
    for n in 1..=last_num {
        sum = sum.add(big_n.pow(n));

        assert_eq!(big_n, BigUint::from(n));
        big_n = big_n.add(&one);
    }

    let string_sum = sum.to_string();
    string_sum[string_sum.len() - 10..].parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 0405071317)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 9110846700)
    }
}
