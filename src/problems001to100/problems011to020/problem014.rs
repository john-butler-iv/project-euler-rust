// https://projecteuler.net/problem=14

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Longest Collatz Sequence",
        number: 14,
        solve: || core_solve(1_000_000),
    }
}

fn core_solve(limit: usize) -> i64 {
    const CACHE_SIZE: usize = 1_000_000;
    let mut cache: Vec<Option<u64>> = vec![None; CACHE_SIZE];
    //let mut cache: [Option<u64>; CACHE_SIZE] = [None; CACHE_SIZE];

    cache[1] = Some(1);

    let mut longest_run = 1;
    let mut longest_start = 1;
    for n in 2..limit as i64 {
        let current_length = find_collatz_length(&mut cache, n);
        if current_length > longest_run {
            longest_run = current_length;
            longest_start = n;
        }
    }

    longest_start
}

fn find_collatz_length(cache: &mut [Option<u64>], n: i64) -> u64 {
    if n >= cache.len() as i64 {
        return find_collatz_length(cache, next_collatz(n)) + 1;
    }
    match cache[n as usize] {
        Some(length) => length,
        None => {
            cache[n as usize] = Some(find_collatz_length(cache, next_collatz(n)) + 1);
            cache[n as usize].unwrap()
        }
    }
}

fn next_collatz(n: i64) -> i64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        let cache = &mut [None; 15];
        cache[1] = Some(1);
        assert_eq!(super::find_collatz_length(cache, 13), 10)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 837799)
    }
}
