// https://projecteuler.net/problem=74

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Digit Factorial Chains",
        number: 74,
        solve: core_solve,
    }
}

const FACTORIAL_TABLE: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn core_solve() -> i64 {
    const LIMIT: usize = 1_000_000;

    let mut memo: [Option<usize>; 363602] = [None; 363602];

    let mut total_sixty_chains = 0;

    for n in 0..LIMIT {
        let chain_size = compute_chain_size(n, &mut memo);

        if chain_size == 60 {
            total_sixty_chains += 1;
        }
    }

    total_sixty_chains
}

fn compute_chain_size(n: usize, memo: &mut [Option<usize>]) -> usize {
    let mut encountered_numbers: Vec<usize> = Vec::new();

    let mut chained_number = n;
    let mut discovered_cycle_length = None;
    while discovered_cycle_length.is_none()
        && memo
            .get(chained_number)
            .and_then(|v| v.to_owned())
            .is_none()
    {
        for (index, encountered_number) in encountered_numbers.iter().rev().enumerate() {
            if encountered_number == &chained_number {
                discovered_cycle_length = Some(index + 1);
            }
        }
        if discovered_cycle_length.is_none() {
            encountered_numbers.push(chained_number);
            chained_number = digit_factorial(chained_number);
        }
    }

    if let Some(discovered_cycle_length) = discovered_cycle_length {
        for (index, encountered_number) in encountered_numbers.iter().rev().enumerate() {
            if index < discovered_cycle_length {
                // it's important that we at least keep a memo of all cycles.
                memo[*encountered_number] = Some(discovered_cycle_length);
            } else if encountered_number < &memo.len() {
                memo[*encountered_number] = Some(index + 1);
            }
        }

        encountered_numbers.len()
    } else {
        for (index, encountered_number) in encountered_numbers.iter().rev().enumerate() {
            if *encountered_number < memo.len() && memo[*encountered_number].is_none() {
                memo[*encountered_number] = Some(memo[chained_number].unwrap() + index + 1);
            }
        }

        encountered_numbers.len() + memo[chained_number].unwrap()
    }
}

fn digit_factorial(n: usize) -> usize {
    n.to_string()
        .as_bytes()
        .iter()
        .map(|digit| FACTORIAL_TABLE[(digit - b'0') as usize])
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn toy_example() {
        // My algorithm relies on always memoizing all numbers in a cycle, but cargo test
        // doesn't seem to allocate enough memory for that. I've commented out the other tests
        // I wanted to run, but I've verified by running in the main code flow
        let mut memo: [Option<usize>; 50000] = [None; 50000];
        assert_eq!(super::compute_chain_size(145, &mut memo), 1);
        /*
        assert_eq!(super::compute_chain_size(169, &mut memo), 3);
        assert_eq!(super::compute_chain_size(363601, &mut memo), 3);
        assert_eq!(super::compute_chain_size(1454, &mut memo), 3);
        // */
        /*
        assert_eq!(super::compute_chain_size(871, &mut memo), 2);
        assert_eq!(super::compute_chain_size(45361, &mut memo), 2);
        // */
        /*
        assert_eq!(super::compute_chain_size(872, &mut memo), 2);
        assert_eq!(super::compute_chain_size(45362, &mut memo), 2);
        // */
        /*
        assert_eq!(super::compute_chain_size(69, &mut memo), 5);
        assert_eq!(super::compute_chain_size(363600, &mut memo), 4);
        // */
        assert_eq!(super::compute_chain_size(78, &mut memo), 4);
        assert_eq!(super::compute_chain_size(45360, &mut memo), 3);
        assert_eq!(super::compute_chain_size(540, &mut memo), 2);
    }

    #[test]
    fn verify_answer() {
        // verifying the answer require more memory than cargo test allows
        //assert_eq!((super::make().solve)(), 402);
    }
}
