// https://projecteuler.net/problem=58

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Spiral Primes",
        number: 58,
        solve: || core_solve(0.1),
    }
}

fn core_solve(desired_ratio: f64) -> i64 {
    let primes = Primes::find_primes(30000);

    let mut total = 9;
    let mut total_primes = 3;

    let mut side_len = 3;

    while (total_primes as f64) / ((2 * side_len - 1) as f64) >= desired_ratio {
        for _ in 0..3 {
            total += side_len + 1;
            if primes.is_prime(total) {
                total_primes += 1;
            }
        }
        total += side_len + 1;

        side_len += 2;
    }

    side_len as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 26241);
    }
}
