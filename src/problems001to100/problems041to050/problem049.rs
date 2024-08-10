// https://projecteuler.net/problem=49

use crate::euler_tools::{prime_finder::Primes, DigitIterator};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "template",
        number: 49,
        solve: || core_solve(1487),
    }
}

fn core_solve(starting_num: u32) -> i64 {
    let primes = Primes::find_primes(10000);

    // there are 168 primes under 1000, and we konw our first number must be four digits
    // we also know we're going to take another number in the while loop below, so we
    // remove one
    let mut prime_iter = primes.prime_iterator().skip(167);
    // technically this will skip the first number we could possibly check, but if input
    // a prime, then that's okay
    assert!(starting_num < 1000 || primes.is_prime(starting_num));
    while *prime_iter.next().unwrap() < starting_num {}

    for num1 in prime_iter {
        let num1_digit_set = DigitIterator::<u32>::create_digit_set(*num1);
        for addend in (2..=5000 - num1 / 2).step_by(2) {
            let num2 = num1 + addend;
            let num3 = num2 + addend;
            if !primes.is_prime(num2) || !primes.is_prime(num3) {
                continue;
            }
            let num2_digit_set = DigitIterator::<u32>::create_digit_set(num2);
            if !DigitIterator::<u32>::are_digital_permutations(&num1_digit_set, &num2_digit_set) {
                continue;
            }
            let num3_digit_set = DigitIterator::<u32>::create_digit_set(num3);
            if !DigitIterator::<u32>::are_digital_permutations(&num1_digit_set, &num3_digit_set) {
                continue;
            }
            return *num1 as i64 * 1_0000_0000 + num2 as i64 * 1_0000 + num3 as i64;
        }
    }
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(0), 1487_4817_8147)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 2969_6299_9629)
    }
}
