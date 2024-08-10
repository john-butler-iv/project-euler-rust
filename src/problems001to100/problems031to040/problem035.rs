// https://projecteuler.net/problem=35

use std::collections::HashSet;

use crate::euler_tools::{prime_finder::Primes, DigitIterator};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Circular Primes",
        number: 35,
        solve: || core_solve(1_000_000),
    }
}

fn check_circular_primes(prime: u32, counted_set: &mut HashSet<u32>, primes: &Primes) {
    let digits: Vec<u32> = DigitIterator::<u32>::new(prime).collect();
    let mut rotated_digits = vec![prime];
    for starting_index in 1..digits.len() {
        let num = DigitIterator::<u32>::combine_digits_rotated(&digits, starting_index);
        if !primes.is_prime(num) {
            return;
        }
        rotated_digits.push(num);
    }
    counted_set.extend(rotated_digits);
}

fn core_solve(limit: usize) -> i64 {
    let primes = Primes::find_primes(limit);

    let mut counted_set: HashSet<u32> = HashSet::new();

    for prime in primes.prime_iterator() {
        if counted_set.contains(prime) {
            continue;
        }
        check_circular_primes(*prime, &mut counted_set, &primes);
    }

    counted_set.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(100), 13)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 55)
    }
}
