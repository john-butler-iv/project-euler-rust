// https://projecteuler.net/problem=57

use num_bigint::BigUint;
use num_traits::One;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Square Root Convergents",
        number: 57,
        solve: || core_solve(1000),
    }
}

fn core_solve(max_expansion: usize) -> i64 {
    let mut numerator: BigUint = BigUint::one();
    let mut denominator: BigUint = BigUint::one();

    let mut big_numerators = 0;

    for _ in 1..=max_expansion {
        // a_n     / b_n     =                 1 + 1 / (2 + 1 / (2 + 1 / (2 + ...)))
        // a_{n+1} / b_{n+1} =  1 + 1 / (      2 + 1 / (2 + 1 / (2 + 1 / (2 + ...))))
        // a_{n+1} / b_{n+1} =  1 + 1 / ( 1 +  1 + 1 / (2 + 1 / (2 + 1 / (2 + ...))))
        // a_{n+1} / b_{n+1} =  1 + 1 / ( 1 + (1 + 1 / (2 + 1 / (2 + 1 / (2 + ...)))))
        // a_{n+1} / b_{n+1} =  1 + 1 / ( 1 +  a_n / b_n)
        // a_{n+1} / b_{n+1} =  1 + b_n / ( b_n + a_n )
        // a_{n+1} / b_{n+1} =  (a_n + b_n) / (a_n + b_n) + b_n / ( a_n + b_n )
        // a_{n+1} / b_{n+1} =  ((a_n + b_n) + b_n) / ( a_n + b_n )
        // a_{n+1} / b_{n+1} =  (a_n + 2 * b_n) / ( a_n + b_n )
        let new_denominator = &numerator + &denominator;
        let new_numerator = &new_denominator + &denominator;

        if new_numerator.to_string().len() > new_denominator.to_string().len() {
            big_numerators += 1;
        }

        numerator = new_numerator;
        denominator = new_denominator;
    }

    big_numerators
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(8), 1)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 153)
    }
}
