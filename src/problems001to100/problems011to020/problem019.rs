// https://projecteuler.net/problem=19

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Counting Sundays",
        number: 19,
        solve: || core_solve(),
    }
}

fn core_solve() -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(), 0)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 0)
    }
}
