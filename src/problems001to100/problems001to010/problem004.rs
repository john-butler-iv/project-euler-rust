// https://projecteuler.net/problem=4

use crate::euler_tools;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Largest Panindrome Product",
        number: 4,
        solve: || core_solve(3),
    }
}

fn core_solve(digits: u8) -> u64 {
    let mut biggest: u64 = 0;
    let min = 10u64.pow((digits - 1) as u32);
    let max = 10 * min - 1;

    let mut min_b = min;

    for a in (min_b..=max).rev() {
        for b in (min_b..=a).rev() {
            let product = a * b;
            if product > biggest && euler_tools::is_palindrome(&product.to_string()) {
                biggest = product;
                min_b = b + 1;
                break;
            }
        }
    }

    biggest
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_problem() {
        assert_eq!(super::core_solve(2), 9009)
    }

    #[test]
    fn validation_solution() {
        assert_eq!(super::core_solve(3), 906609)
    }
}
