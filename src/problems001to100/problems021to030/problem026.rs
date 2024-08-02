// https://projecteuler.net/problem=26

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Reciprocal Cycles",
        number: 26,
        solve: || core_solve(1000),
    }
}

fn core_solve(max_denom: usize) -> u64 {
    let mut longest_length = 1;
    let mut longest_denom = 1;

    let primes = Primes::find_primes(max_denom);

    let mut remainders: Vec<u32> = Vec::new();

    for denom in primes.prime_iterator() {
        remainders.clear();
        let mut n = 10;
        remainders.push(1);

        let mut remainder_looped = false;
        while !remainder_looped {
            n %= denom;
            if n == 0 {
                break;
            }

            for (i, remainder) in remainders.iter().enumerate() {
                if *remainder == n {
                    remainder_looped = true;
                    let length = remainders.len() - i;
                    if longest_length < length {
                        longest_denom = *denom;
                        longest_length = length;
                    }
                    break;
                }
            }

            remainders.push(n);
            n *= 10;
        }
    }

    longest_denom as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(11), 7)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 983)
    }
}
