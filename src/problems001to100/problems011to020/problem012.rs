// https://projecteuler.net/problem=12

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Highly Divisible Triangular Number",
        number: 12,
        solve: || core_solve(500),
    }
}

fn core_solve(min_divisors: usize) -> i64 {
    // TODO Use an approximation of the inverse of Euler's Totient function
    let limit: usize = 4 * min_divisors * min_divisors;

    let primes = Primes::find_primes(limit);

    let mut triangle = 0u64;
    for n in 1..limit as u64 {
        triangle += n;

        if primes.divisors(&triangle) > min_divisors {
            return triangle as i64;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(5), 28)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 76_576_500)
    }
}
