// https://projecteuler.net/problem=9

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Special Pythagorean Triplet",
        number: 9,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    for a in 1..=998 {
        for b in 1..=(999 - a) {
            if 1000 * (a + b) - a * b == 500000 {
                let c = 1000 - a - b;
                return a * b * c;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    // the toy example in the problem description is to verify your understanding of pythagorean
    // triplets. It is not helpful for verifying methods, so no test for it.

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 31875000)
    }
}
