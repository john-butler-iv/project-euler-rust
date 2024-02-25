use crate::euler_tools::prime_finder::Primes;
use integer_sqrt::IntegerSquareRoot;
pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "Largest Prime Factor",
        number: 3,
        solve: || core_solve(600851475143),
    }
}

fn core_solve(input: u64) -> u64 {
    Primes::find_primes(IntegerSquareRoot::integer_sqrt(
        &usize::try_from(input).expect("usize should be 32 or 64"),
    ))
    .prime_factorize(&input)
    .iter()
    .last()
    .unwrap_or(&input)
    .to_owned()
}

#[cfg(test)]
mod tests {
    use crate::euler_tools::prime_finder::Primes;

    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(13195), 29)
    }

    #[test]
    fn toy_factorization() {
        let primes = Primes::find_primes(13195);

        assert_eq!(primes.prime_factorize(&13195), vec![5, 7, 13, 29]);
    }
}
