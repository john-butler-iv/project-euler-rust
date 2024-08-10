// https://projecteuler.net/problem=47

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Distinct Prime Factors",
        number: 47,
        solve: || core_solve(4),
    }
}

fn core_solve(required_factors: usize) -> i64 {
    let mut num_factors_cache = [0; 200000];

    let mut consec_primes = 0;
    for n in 2..(num_factors_cache.len()) {
        let num_factors = num_factors_cache[n];
        if num_factors == required_factors {
            consec_primes += 1;
            if required_factors == consec_primes {
                return (n - consec_primes + 1) as i64;
            }
        } else {
            consec_primes = 0;
            if num_factors == 0 {
                for multiple in num_factors_cache.iter_mut().skip(2 * n).step_by(n) {
                    *multiple += 1;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(2), 14);
        assert_eq!(super::core_solve(3), 644);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 134043)
    }
}
