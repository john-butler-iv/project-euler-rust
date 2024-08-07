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
    let mut cache: Vec<u64> = vec![0; CACHE_SIZE];

    cache[1] = 1;

    let mut longest_run = 1;
    let mut longest_start = 1;
    for n in 2..limit {
        let current_length = find_collatz_length(&mut cache, n);
        if current_length > longest_run {
            longest_run = current_length;
            longest_start = n;
        }
    }

    longest_start as i64
}

fn find_collatz_length(cache: &mut [u64], n: usize) -> u64 {
    if n >= cache.len() {
        return find_collatz_length(cache, next_collatz(n)) + 1;
    }
    match cache[n] {
        0 => {
            cache[n] = find_collatz_length(cache, next_collatz(n)) + 1;
            cache[n]
        }
        length => length,
    }
}

fn next_collatz(n: usize) -> usize {
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
        let cache = &mut [0; 15];
        cache[1] = 1;
        assert_eq!(super::find_collatz_length(cache, 13), 10)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 837799)
    }
}
