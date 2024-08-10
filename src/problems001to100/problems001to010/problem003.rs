// https://projecteuler.net/problem=3

use crate::euler_tools::prime_finder::Primes;
use integer_sqrt::IntegerSquareRoot;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Largest Prime Factor",
        number: 3,
        solve: || core_solve(600851475143),
    }
}

// this approach runs in 0.017 ms. Factorizing primes is just too slow
fn core_solve(input: i64) -> i64 {
    let mut remainder = input;

    // 2 is not divisible, so we can just skip it
    for i in (3..input).step_by(2) {
        while remainder % i == 0 {
            remainder /= i;
        }
        if remainder == 1 {
            return i;
        }
    }
    input
}

// fastest run was in 3.892 ms. If we lower the limit to just above the answer,
// we can run in 0.029 ms, but I don't have any reason to use that number other
// than it just is the answer, so I fell like I'm cheating when I use it.
// Also that's still slower than the fast approach
#[allow(dead_code)]
fn core_solve_slow(input: u64) -> i64 {
    Primes::find_primes(IntegerSquareRoot::integer_sqrt(&(input as usize)))
        .prime_factorize(input)
        .iter()
        .last()
        .unwrap_or(&input)
        .to_owned() as i64
}

#[cfg(test)]
mod tests {
    use crate::euler_tools::prime_finder::Primes;

    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(13195), 29)
    }

    #[test]
    fn toy_answers_match() {
        assert_eq!(super::core_solve(13195), super::core_solve_slow(13195))
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 6857);
    }

    #[test]
    fn verify_real_answers_match() {
        assert_eq!(
            super::core_solve(600851475143),
            super::core_solve_slow(600851475143)
        );
    }

    #[test]
    fn toy_factorization() {
        let primes = Primes::find_primes(13195);

        assert_eq!(primes.prime_factorize(13195), vec![5, 7, 13, 29]);
    }
}
