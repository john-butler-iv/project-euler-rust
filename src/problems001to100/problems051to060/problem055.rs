// https://projecteuler.net/problem=55

use std::str::FromStr;

use num_bigint::BigUint;
use num_traits::FromPrimitive;

use crate::euler_tools::is_palindrome;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Lychrel Numbers",
        number: 55,
        solve: core_solve,
    }
}

const LIMIT: usize = 10_000usize;
fn core_solve() -> i64 {
    let big_limit: BigUint = BigUint::from_usize(LIMIT).expect("usize can always fit in a BigUint");
    let mut is_lychrel_cache: [Option<bool>; LIMIT] = [None; LIMIT];

    (0..LIMIT)
        .filter(|n| compute_is_lychrel(*n, &mut is_lychrel_cache, &big_limit))
        .count() as i64
}

fn compute_is_lychrel(
    n: usize,
    is_lychrel_cache: &mut [Option<bool>; LIMIT],
    big_limit: &BigUint,
) -> bool {
    compute_is_lychrel_core(
        BigUint::from_usize(n).expect("usize can always fit in a BigUint"),
        is_lychrel_cache,
        0,
        big_limit,
    )
}

fn compute_is_lychrel_core(
    n: BigUint,
    is_lychrel_cache: &mut [Option<bool>; LIMIT],
    depth: usize,
    big_limit: &BigUint,
) -> bool {
    let reversed_n = BigUint::from_str(unsafe {
        &String::from_utf8_unchecked(format!("{n}").bytes().rev().collect::<Vec<u8>>())
    })
    .expect("string built from manipulated digits should still be a valid uint");
    let next_n = &n + &reversed_n;

    let check_can_index = |n: &BigUint| {
        (
            if n == &BigUint::ZERO {
                0usize
            } else {
                n.to_u64_digits()[0] as usize
            },
            n < big_limit,
        )
    };

    let (n_index, n_can_index) = check_can_index(&n);
    let (next_n_index, next_n_can_index) = check_can_index(&next_n);
    let (reversed_n_index, reversed_n_can_index) = check_can_index(&reversed_n);

    let is_lychrel = if next_n_can_index && is_lychrel_cache[next_n_index].is_some() {
        // I would use an if let, but with the extra bounds check, it doesn't work
        is_lychrel_cache[next_n_index].expect("just checked is_some")
    } else if depth >= (50 - 1) {
        true
    } else if is_palindrome(&format!("{next_n}")) {
        false
    } else {
        compute_is_lychrel_core(next_n, is_lychrel_cache, depth + 1, big_limit)
    };
    if n_can_index {
        is_lychrel_cache[n_index] = Some(is_lychrel);
    }
    if reversed_n_can_index {
        is_lychrel_cache[reversed_n_index] = Some(is_lychrel);
    }
    is_lychrel
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn check_47() {
        let big_limit = BigUint::from_usize(super::LIMIT).unwrap();
        let mut cache = [None; super::LIMIT];

        assert!(!super::compute_is_lychrel(47, &mut cache, &big_limit));
        assert!(cache[47].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[74].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[121].is_none());
    }

    #[test]
    fn check_349() {
        let big_limit = BigUint::from_usize(super::LIMIT).unwrap();
        let mut cache = [None; super::LIMIT];

        assert!(!super::compute_is_lychrel(349, &mut cache, &big_limit));
        assert!(cache[349].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[943].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[1292].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[2921].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[4213].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[3124].is_some_and(|is_lychrel| !is_lychrel));
        assert!(cache[7337].is_none());
    }

    #[test]
    fn check_196() {
        let big_limit = BigUint::from_usize(super::LIMIT).unwrap();
        let mut cache = [None; super::LIMIT];

        assert!(super::compute_is_lychrel(196, &mut cache, &big_limit));
        assert!(cache[196].is_some_and(|is_lychrel| is_lychrel));
    }

    #[test]
    fn check_4994() {
        let big_limit = BigUint::from_usize(super::LIMIT).unwrap();
        let mut cache = [None; super::LIMIT];

        assert!(super::compute_is_lychrel(4994, &mut cache, &big_limit));
        assert!(cache[4994].is_some_and(|is_lychrel| is_lychrel));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 249)
    }
}
