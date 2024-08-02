// https://projecteuler.net/problem=5

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Smallest Multiple",
        number: 5,
        solve: || core_solve(20),
    }
}

fn core_solve(limit: usize) -> u64 {
    let primes = Primes::find_primes(limit + 1);

    let mut total_factor_multiplicity: Vec<u32> = vec![0; limit + 1];

    for num in 2..limit {
        let num_factors = primes.prime_factorize(&u64::try_from(num).expect(
            "if we run into capacity concerns, this algorithm will take too long to run anyway",
        ));

        let mut current_factor_multiplicity = vec![0; limit + 1];
        for p in num_factors {
            let p = usize::try_from(p).expect("p < limit becuase p is a factor of limit");
            current_factor_multiplicity[p] += 1
        }

        for (p, multiplicity) in current_factor_multiplicity.iter().enumerate() {
            total_factor_multiplicity[p] =
                std::cmp::max(multiplicity.to_owned(), total_factor_multiplicity[p]);
        }
    }

    let mut least_common_multiple = 1;
    for (p, multiplicity) in total_factor_multiplicity
        .iter()
        .enumerate()
        .filter(|(_, mult)| **mult != 0)
    {
        least_common_multiple *= p.pow(multiplicity.to_owned());
    }
    u64::try_from(least_common_multiple).expect(
        "if running on inputs where this is an issue, we probably won't even get to this point",
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 2520);
    }

    #[test]
    fn validate_solution() {
        assert_eq!((super::make().solve)(), 232792560);
    }
}
