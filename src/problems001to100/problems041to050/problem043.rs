// https://projecteuler.net/problem=43

use crate::euler_tools::{
    collection_tools::{inplace_permute, PermutationStatus},
    DigitIterator,
};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Sub-String Divisibility",
        number: 43,
        solve: core_solve,
    }
}

fn is_substring_divisible(digits: &[i64]) -> bool {
    // digits are little-endian, so the indexes don't match the problem
    digits[6] & 1 == 0
        && (digits[5] + digits[6] + digits[7]) % 3 == 0
        && (digits[4] == 0 || digits[4] == 5)
        && DigitIterator::<i64>::combine_digits(&digits[3..=5]) % 7 == 0
        && DigitIterator::<i64>::combine_digits(&digits[2..=4]) % 11 == 0
        && DigitIterator::<i64>::combine_digits(&digits[1..=3]) % 13 == 0
        && DigitIterator::<i64>::combine_digits(&digits[..=2]) % 17 == 0
}

fn core_solve() -> i64 {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut sum = 0;
    while inplace_permute(&mut digits) != PermutationStatus::Finished {
        if is_substring_divisible(&digits) {
            sum += DigitIterator::<i64>::combine_digits(&digits);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert!(super::is_substring_divisible(&[9, 8, 2, 7, 5, 3, 6, 0, 4]))
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 16695334890)
    }
}
