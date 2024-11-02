// https://projecteuler.net/problem=52

use crate::euler_tools::DigitIterator;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Permuted Multiples",
        number: 52,
        solve: || core_solve(6),
    }
}

fn core_solve(highest_multiple: i64) -> i64 {
    for digit_length in 1.. {
        let largest_base_value = 10i64.pow(digit_length + 1) / highest_multiple;

        for base_value in 10i64.pow(digit_length)..=largest_base_value {
            let mut all_values_match = true;
            let base_set = DigitIterator::<i64>::create_digit_set(base_value);
            for multiple in 2..=highest_multiple {
                let product = base_value * multiple;

                if !DigitIterator::<i64>::create_digit_set(product)
                    .iter()
                    .zip(base_set.iter())
                    .all(|(a, b)| a == b)
                {
                    all_values_match = false;
                    break;
                }
            }

            if all_values_match {
                return base_value;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(2), 125874)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 142857)
    }
}
