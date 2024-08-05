// https://projecteuler.net/problem=32

use std::collections::HashSet;

use num_traits::pow;

use crate::euler_tools::DigitIterator;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Pandigital Products",
        number: 32,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let mut sum = 0;
    let mut obtained_nums: HashSet<usize> = HashSet::new();

    let mut a = 1;
    while a < 10_000 {
        let mut a_digit_cache = [0usize; 10];
        let a_digit_base = match find_repeating_digit(&mut a_digit_cache, a, 0) {
            PandigitalCheckResults::HasRepeat(digit_offset) => {
                a += digit_offset;
                continue;
            }
            PandigitalCheckResults::AllUnique(total_digits) => total_digits,
        };

        let mut b = a + 1;
        while b < 10_000 {
            let mut b_digit_cache = a_digit_cache;
            let b_digit_base = match find_repeating_digit(&mut b_digit_cache, b, a_digit_base) {
                PandigitalCheckResults::HasRepeat(digit_offset) => {
                    b += digit_offset;
                    continue;
                }
                PandigitalCheckResults::AllUnique(total_digits) => total_digits,
            };

            let c = a * b;
            if !obtained_nums.contains(&c) {
                if let PandigitalCheckResults::AllUnique(total_digits) =
                    find_repeating_digit(&mut b_digit_cache, c, b_digit_base)
                {
                    if total_digits == 9 {
                        sum += c;
                        obtained_nums.insert(c);
                    }
                }
            }
            b += 1;
        }
        a += 1;
    }

    sum as i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PandigitalCheckResults {
    AllUnique(usize), // the number of digits
    HasRepeat(usize), // the number next offset to use.
}

fn find_repeating_digit(
    digit_cache: &mut [usize; 10],
    n: usize,
    digit_base: usize,
) -> PandigitalCheckResults {
    let mut current_digit_index = digit_base + 1;
    for (index, digit) in DigitIterator::new(n).enumerate() {
        current_digit_index = index + digit_base + 1;

        if digit == 0 {
            return PandigitalCheckResults::HasRepeat(pow(10, index));
        } else if digit_cache[digit] == 0 {
            digit_cache[digit] = current_digit_index;
        } else if digit_cache[digit] <= digit_base {
            return PandigitalCheckResults::HasRepeat(pow(10, index));
        } else {
            return PandigitalCheckResults::HasRepeat(pow(10, digit_cache[digit] - 1 - digit_base));
        }
    }
    PandigitalCheckResults::AllUnique(current_digit_index)
}

#[cfg(test)]
mod tests {
    use crate::problems001to100::problems031to040::problem032::PandigitalCheckResults;

    use super::find_repeating_digit;

    #[test]
    fn example_product_is_pandigital() {
        let mut digit_cache = [0usize; 10];

        let a_result = find_repeating_digit(&mut digit_cache, 39, 0);
        assert_eq!(a_result, PandigitalCheckResults::AllUnique(2));
        assert_eq!(digit_cache, [0, 0, 0, 2, 0, 0, 0, 0, 0, 1]);

        let b_result = find_repeating_digit(&mut digit_cache, 186, 2);
        assert_eq!(b_result, PandigitalCheckResults::AllUnique(5));
        assert_eq!(digit_cache, [0, 5, 0, 2, 0, 0, 3, 0, 4, 1]);

        let c_result = find_repeating_digit(&mut digit_cache, 7254, 5);
        assert_eq!(c_result, PandigitalCheckResults::AllUnique(9));
        assert_eq!(digit_cache, [0, 5, 8, 2, 6, 7, 3, 9, 4, 1]);
    }

    #[test]
    fn example_is_pandigital() {
        let mut digit_cache = [0usize; 10];

        let result = find_repeating_digit(&mut digit_cache, 15234, 0);
        assert_eq!(result, PandigitalCheckResults::AllUnique(5));
        assert_eq!(digit_cache, [0, 5, 3, 2, 1, 4, 0, 0, 0, 0])
    }

    #[test]
    fn simple_is_not_pandigital() {
        let mut digit_cache = [0usize; 10];

        let result = find_repeating_digit(&mut digit_cache, 123456178, 0);
        assert_eq!(result, PandigitalCheckResults::HasRepeat(100));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 45228)
    }
}
