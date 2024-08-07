// https://projecteuler.net/problem=34

use crate::euler_tools::{self, DigitIterator};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Digit Factorials",
        number: 34,
        solve: core_solve,
    }
}

fn compute_digit_sum(factorial_table: &[i64], n: i64) -> i64 {
    DigitIterator::<i64>::new(n)
        .map(|digit| factorial_table[digit as usize])
        .sum()
}

fn core_solve() -> i64 {
    let factorial_table = euler_tools::sized_factorial_table::<i64>(10);

    (10..=2540160)
        .filter(|n| compute_digit_sum(&factorial_table, *n) == *n)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(
            super::compute_digit_sum(&crate::euler_tools::sized_factorial_table(10), 145),
            145
        )
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 40730)
    }
}
