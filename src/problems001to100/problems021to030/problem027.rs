// https://projecteuler.net/problem=27

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Quadratic Primes",
        number: 27,
        solve: || core_solve(1000),
    }
}

fn core_solve(limit: usize) -> i64 {
    let primes = Primes::find_primes(13000);

    let mut best_a = 0;
    let mut best_b = 0;
    let mut most_primes = 0;

    let mut check_best = |a: &i64, b: &i64| {
        let consec_primes = count_consecutive_primes(&primes, *a, *b);
        if consec_primes > most_primes {
            most_primes = consec_primes;
            best_a = *a;
            best_b = *b;
        }
    };

    for b in primes
        .bounded_prime_iterator(limit as u32 + 1)
        .map(|b| *b as i64)
    {
        for a in 0..limit as i64 {
            check_best(&a, &b);
            check_best(&a, &-b);
            check_best(&-a, &b);
            check_best(&-a, &-b);
        }
    }

    best_a * best_b
}

fn count_consecutive_primes(primes: &Primes, a: i64, b: i64) -> i64 {
    for n in 0.. {
        let quad_value = n * n + a * n + b;
        if quad_value < 0 {
            return n;
        }
        if !primes.is_prime(&(quad_value as u32)) {
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::euler_tools::prime_finder::Primes;

    #[test]
    fn toy_examples() {
        let primes = Primes::find_primes(100000);
        assert_eq!(super::count_consecutive_primes(&primes, 1, 41), 40);
        assert_eq!(super::count_consecutive_primes(&primes, -79, 1601), 80);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), -59231)
    }
}
