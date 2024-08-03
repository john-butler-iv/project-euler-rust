// https://projecteuler.net/problem=

use crate::euler_tools::{self, prime_finder::Primes};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "10,001st Prime",
        number: 7,
        solve: || core_solve(10_001),
    }
}

fn core_solve(prime_index: usize) -> i64 {
    // primes under n = pi(n) = ~ n / ln (n), which I will treat as given for this problem.
    // we want the inverse of this, but there's no elementary inverse :(
    // => n = ~ -pi W(-1/pi) where W is the Lambert W function (inverse of xe^x) according to Wolfram Alpha
    // remember pi is the number of primes under a certain value.

    // Because xe^x decreases then increases again, W(-1/e) to W(0) has two outputs.
    // W_0(x) is the branch that increases logorithmically and has slope 1 as it passes through x=0
    // W_{-1}(x) is the branch that is only defined for real numbers for W(-1/e) to W(0) and decreases
    // asymptotically to -infinity as x approaches 0.

    // Knowing that n is supposed to grow and grow as pi increases, we should be able to see that
    // we want W(-1/pi) to approach -infinity as pi -> infinity
    // <=> we want W(x) to approach -infinity as x -> 0 from the negative direction
    // therefore, we want the W_{-1} branch.

    let pi = prime_index as f64;
    let mut approx = (-pi * euler_tools::lambert_w_m1_neg_inv(pi)) as usize;

    let mut primes = Primes::find_primes(approx);

    // This check is more for safety. It's only needed for relatively small prime_index values, such as our testing value.
    // NOTE: pi(n) > n / ln(n) for n >= 1250, although 1250 isn't the lowest upper bound
    // so n / ln(n) is an under estimation of pi(n)
    // ALSO NOTE: pi(x) and x / ln (x) are both strictly increasing functions, so when we find the inverse,
    // our approximation is an over estimation of pi(n) if n >= 1250
    while primes.total_primes() < prime_index {
        approx *= 2;
        primes = Primes::find_primes(approx);
    }

    primes.get_prime(prime_index - 1).unwrap_or(&0).to_owned() as i64
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
