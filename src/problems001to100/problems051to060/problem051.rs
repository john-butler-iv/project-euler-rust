// https://projecteuler.net/problem=51

use radix_fmt::radix;

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Prime Digital Replacements",
        number: 51,
        solve: || core_solve(8),
    }
}

fn core_solve(num_primes: usize) -> i64 {
    let primes = Primes::find_primes(2000000);

    for base_num in i64::from_str_radix("56aa3", 11).expect("literal value").. {
        let base_pattern = radix(base_num, 11).to_string();
        if !base_pattern.contains('a') {
            continue;
        }
        let mut first_prime_in_pattern: Option<i64> = None;

        let digit_replacement_primes: usize = (0..=9)
            .map(|digit| {
                let mut total = 0u32;
                for char in base_pattern.as_bytes() {
                    total = 10 * total
                        + match char {
                            b'a' => digit,
                            other => (other - b'0') as u32,
                        }
                }
                total
            })
            .filter(|number| {
                let filter_value = primes.is_prime(*number);
                if filter_value && first_prime_in_pattern.is_none() {
                    first_prime_in_pattern = Some(*number as i64);
                }
                filter_value
            })
            .count();
        if digit_replacement_primes >= num_primes {
            // there ought to be a first prime if we've surpassed num_primes. If that's
            // false, it requires a different approach
            return first_prime_in_pattern.unwrap();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(7), 56003)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 121313)
    }
}
