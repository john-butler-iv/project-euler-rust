// https://projecteuler.net/problem=

use crate::euler_tools::{self, Primes};

pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "10,001st Prime",
        number: 7,
        solve: || core_solve(10_001),
    }
}

fn core_solve(prime_index: u32) -> u64 {
    // primes under n = pi(n) = ~ n / ln (n)
    // we want the inverse of this, but there's no elementary inverse :(
    // => n = ~ -x W(-1/x) where W is the Lambert W function (inverse of xe^x)

    let Primes = Primes::find_primes(3 * prime_index);
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(6), 13)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 104743)
    }
}
