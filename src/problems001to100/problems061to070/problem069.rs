// https://projecteuler.net/problem=69

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Totient Maximum",
        number: 69,
        solve: || core_solve(1_000_000),
    }
}

fn core_solve(limit: u32) -> i64 {
    let primes = Primes::find_primes((limit as f64).log2().floor() as usize + 1);

    let mut n = 1;

    for prime in primes.prime_iterator() {
        let next_n = n * prime;
        if next_n > limit {
            break;
        }
        n = next_n;
    }

    n as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 6);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 510510);
    }
}
