// https://projecteuler.net/problem=21

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Amicable Numbers",
        number: 21,
        solve: || core_solve(10_000),
    }
}

fn core_solve(amicables_under: u64) -> u64 {
    let mut amicable_sum = 0;
    let pf = Primes::find_primes(amicables_under as usize);

    for i in 3..amicables_under {
        let sig_i = pf.sigma(&i);
        let j = sig_i - i;
        if i == j {
            continue;
        }

        let sig_j = pf.sigma(&j);
        if sig_j == sig_i {
            amicable_sum += i;
        }
    }

    amicable_sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(300), 220 + 284)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 31626)
    }
}
