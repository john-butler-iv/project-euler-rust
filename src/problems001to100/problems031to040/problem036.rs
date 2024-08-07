// https://projecteuler.net/problem=36

use crate::euler_tools;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Double-Base Palindromes",
        number: 36,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    (1usize..1_000_000)
        .filter(|n| {
            // faster to do base-2 first because bit manipulation/a single equality check is way faster than the other approach
            euler_tools::is_bin_palindrome(*n) && euler_tools::is_palindrome(&n.to_string())
        })
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 872187)
    }
}
