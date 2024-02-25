// https://projecteuler.net/problem=

use crate::euler_tools;

pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "template",
        number: 0,
        solve: || core_solve(),
    }
}

fn core_solve() -> u64 {
    0u64
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
