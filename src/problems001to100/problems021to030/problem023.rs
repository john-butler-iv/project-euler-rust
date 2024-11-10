// https://projecteuler.net/problem=23

use std::cmp::Ordering;

use crate::euler_tools::{figurate_numbers::Triangle, prime_finder::Primes};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Non-Abundant Sums",
        number: 23,
        solve: core_solve_fast,
    }
}

const MAX_NON_ABUNDANT_SUM: u32 = 20161; // 28123 is the given value, but 20161 is known to be the real maximum value

// this approach runs in 34 ms on my machine with the release flag set.
#[allow(dead_code)]
fn core_solve_slow() -> i64 {
    let primes = Primes::find_primes((MAX_NON_ABUNDANT_SUM - 12) as usize);
    let abundant_numbers: Vec<u32> = (12..MAX_NON_ABUNDANT_SUM)
        .filter(|n| primes.is_abundant(*n as u64))
        .collect();

    let mut sum = Triangle::triangle(MAX_NON_ABUNDANT_SUM) as i64;
    let mut upper_index_bound = 3;
    for n in 24..=MAX_NON_ABUNDANT_SUM {
        if upper_index_bound < abundant_numbers.len() - 1 && abundant_numbers[upper_index_bound] < n
        {
            upper_index_bound += 1;
        }
        let mut lower_index = 0;
        let mut higher_index = upper_index_bound;
        while lower_index <= higher_index {
            match n.cmp(&(abundant_numbers[lower_index] + abundant_numbers[higher_index])) {
                Ordering::Less => higher_index -= 1,
                Ordering::Greater => lower_index += 1,
                Ordering::Equal => {
                    sum -= n as i64;
                    break;
                }
            }
        }
    }

    sum
}

// this approach runs in 22 ms on my machine with the release flag set.
fn core_solve_fast() -> i64 {
    let primes = Primes::find_primes((MAX_NON_ABUNDANT_SUM - 12) as usize);
    let abundant_nums: Vec<u32> = (12..MAX_NON_ABUNDANT_SUM)
        .filter(|n| primes.is_abundant(*n as u64))
        .collect();

    let mut can_be_expressed = vec![false; MAX_NON_ABUNDANT_SUM as usize + 1];
    let mut sum = Triangle::triangle(MAX_NON_ABUNDANT_SUM) as i64;
    for (index, abundant_num1) in abundant_nums.iter().enumerate() {
        for abundant_num2 in abundant_nums[index..].iter() {
            if abundant_num1 + abundant_num2 <= MAX_NON_ABUNDANT_SUM
                && !can_be_expressed[(abundant_num1 + abundant_num2) as usize]
            {
                sum -= (abundant_num1 + abundant_num2) as i64;
                can_be_expressed[(abundant_num1 + abundant_num2) as usize] = true;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    // there are no real tests other than the ones written in prime_finder
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 4179871)
    }
}
