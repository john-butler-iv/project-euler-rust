// https://projecteuler.net/problem=30

use num_traits::pow;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Digit Fifth Powers",
        number: 30,
        solve: || core_solve(5),
    }
}

fn core_solve(exponent: usize) -> i64 {
    let mut sum = 0;

    for n in 2..=999999 {
        let mut digit_power_sum = 0;
        let mut digits = n;
        while digits > 0 {
            let digit = digits % 10;
            digits /= 10;

            digit_power_sum += pow(digit, exponent);
        }
        if digit_power_sum == n {
            sum += n
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(4), 19316)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 443839)
    }
}
