// https://projecteuler.net/problem=20

use num_bigint::BigUint;

use crate::euler_tools;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Factorial Digit Sum",
        number: 20,
        solve: || core_solve(100),
    }
}

fn core_solve(digits: u8) -> i64 {
    euler_tools::big_factorial(BigUint::from(digits))
        .to_string()
        .as_bytes()
        .iter()
        .map(|n| i64::from(n - b'0'))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 27)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 648)
    }
}
