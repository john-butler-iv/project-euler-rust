// https://projecteuler.net/problem=16

use num_bigint::BigUint;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Power Digit Sum",
        number: 16,
        solve: || core_solve(1000),
    }
}

fn core_solve(exponent: u32) -> u64 {
    let large_number = BigUint::from(2usize);

    large_number
        .pow(exponent)
        .to_str_radix(10)
        .as_bytes()
        .iter()
        .fold(0, |total, digit| total + (digit - b'0') as u64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(15), 26)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 1366)
    }
}
