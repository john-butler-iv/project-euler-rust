// https://projecteuler.net/problem=50

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Consecutive Prime Sum",
        number: 50,
        solve: || core_solve(1_000_000),
    }
}

fn core_solve(limit: u32) -> i64 {
    let primes = Primes::find_primes(limit as usize);

    let max_consec_primes: usize = (|| {
        let mut sum = 0;
        for (prime_index, prime) in primes.prime_iterator().enumerate() {
            sum += prime;
            if sum >= limit {
                return prime_index;
            }
        }
        primes.total_primes()
    })();

    for num_primes in (1..=max_consec_primes).rev() {
        let mut current_prim_iter = primes.prime_iterator();
        let mut sum: u32 = (&mut current_prim_iter).take(num_primes).sum();

        if primes.is_prime_basic(sum) {
            return sum as i64;
        }
        for (higher_prime, lower_prime) in current_prim_iter.zip(primes.prime_iterator()) {
            sum += higher_prime - lower_prime;
            if sum > limit {
                break;
            }
            if primes.is_prime_basic(sum) {
                return sum as i64;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(100), 41);
        assert_eq!(super::core_solve(1000), 953);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 997651);
    }
}
