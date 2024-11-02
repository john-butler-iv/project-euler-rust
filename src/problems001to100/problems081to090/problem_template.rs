// https://projecteuler.net/problem=8

use crate::euler_tools::{self, prime_finder::Primes};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "template",
        number: 8,
        solve: || core_solve(),
    }
}

fn core_solve() -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(), 0);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 0);
    }
}
