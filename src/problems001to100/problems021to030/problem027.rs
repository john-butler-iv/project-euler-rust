// https://projecteuler.net/problem=27

//use crate::euler_tools::{self, prime_finder::Primes};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Quadratic Primes",
        number: 27,
        solve: || core_solve(1),
    }
}

fn core_solve(_: usize) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(1), 0)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 0)
    }
}
