// https://projecteuler.net/problem=56

use std::cmp::max;

use num_bigint::BigUint;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Powerful Digit Sum",
        number: 56,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let mut biggest_digit_sum = 0;
    for a in 2u32..100 {
        let big_a = BigUint::from(a);
        for b in 2..100 {
            biggest_digit_sum = max(biggest_digit_sum, big_digit_sum(&big_a, b));
        }
    }

    biggest_digit_sum
}

fn big_digit_sum(a: &BigUint, b: u32) -> i64 {
    let big_num = a.pow(b);
    big_num
        .to_str_radix(10)
        .as_bytes()
        .iter()
        .map(|byte| (byte - b'0') as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    #[test]
    fn toy_example() {
        assert_eq!(super::big_digit_sum(&BigUint::from(10u32), 100), 1);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 972)
    }
}
