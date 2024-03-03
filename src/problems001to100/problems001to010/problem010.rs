// https://projecteuler.net/problem=10

use crate::euler_tools::prime_finder;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Summation of Primes",
        number: 10,
        solve: || core_solve(2_000_000),
    }
}

fn core_solve(limit: usize) -> u64 {
    prime_finder::Primes::find_primes(limit)
        .prime_iterator()
        // convert to u64 to prevent overflow
        .map(|p| *p as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 17)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 142913828922)
    }
}
