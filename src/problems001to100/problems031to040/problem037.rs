// https://projecteuler.net/problem=3

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Truncatable Primes",
        number: 37,
        solve: core_solve,
    }
}

fn is_right_truncatable(n: u32, primes: &Primes) -> bool {
    let mut n = n / 10;
    while n > 0 {
        if !primes.is_prime(&n) {
            return false;
        }
        n /= 10;
    }
    true
}

fn is_left_truncatable(n: u32, biggest_digit: u32, primes: &Primes) -> bool {
    let mut biggest_digit = biggest_digit / 10;
    while biggest_digit > 1 {
        if !primes.is_prime(&(n % biggest_digit)) {
            return false;
        }
        biggest_digit /= 10;
    }
    true
}

fn core_solve() -> i64 {
    let mut biggest_digit = 100;
    let primes = Primes::find_primes(1_000_000);

    let mut sum = 0;

    let mut primes_found = 11;

    for prime in primes.prime_iterator().skip(4) {
        // 2, 3, 5, 7 explicitly disallowed
        if !is_right_truncatable(*prime, &primes) {
            continue;
        }
        while biggest_digit < *prime {
            biggest_digit *= 10;
        }
        if !is_left_truncatable(*prime, biggest_digit, &primes) {
            continue;
        }
        sum += prime;
        primes_found += 1;
        if primes_found == 11 {
            break;
        }
    }
    sum as i64
}

#[cfg(test)]
mod tests {
    use crate::{
        euler_tools::prime_finder::Primes,
        problems001to100::problems031to040::problem037::{
            is_left_truncatable, is_right_truncatable,
        },
    };

    #[test]
    fn right_truncatable() {
        let primes = Primes::find_primes(5000);
        assert!(is_right_truncatable(37, &primes));
        assert!(is_right_truncatable(379, &primes));
        assert!(is_right_truncatable(3797, &primes));

        assert!(!is_right_truncatable(43, &primes));
    }
    #[test]
    fn left_truncatable() {
        let primes = Primes::find_primes(5000);
        assert!(is_left_truncatable(97, 100, &primes));
        assert!(is_left_truncatable(797, 1000, &primes));
        assert!(is_left_truncatable(3797, 10000, &primes));

        assert!(!is_left_truncatable(521, 1000, &primes));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 748317)
    }
}
