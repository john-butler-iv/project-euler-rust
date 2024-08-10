// https://projecteuler.net/problem=46

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Goldbach's Other Conjecture",
        number: 46,
        solve: core_solve,
    }
}

fn is_representable(n: u32, primes: &Primes) -> bool {
    for sqrt in 1.. {
        let square_term = 2 * sqrt * sqrt;
        if square_term >= n {
            return false;
        }
        let remainder = n - square_term;
        if primes.is_prime(remainder) {
            return true;
        }
    }
    unreachable!()
}

fn core_solve() -> i64 {
    let primes = Primes::find_primes(10000);
    (9..)
        .step_by(2)
        .find(|n| !primes.is_prime(*n) && !is_representable(*n, &primes))
        .expect("we're told it's there. ") as i64
}

#[cfg(test)]
mod tests {
    use crate::{
        euler_tools::prime_finder::Primes,
        problems001to100::problems041to050::problem046::is_representable,
    };

    #[test]
    fn toy_example() {
        let primes = Primes::find_primes(100);
        assert!(is_representable(19, &primes));
        assert!(is_representable(15, &primes));
        assert!(is_representable(21, &primes));
        assert!(is_representable(25, &primes));
        assert!(is_representable(27, &primes));
        assert!(is_representable(33, &primes));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 5777)
    }
}
