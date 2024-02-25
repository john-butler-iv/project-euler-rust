// https://projecteuler.net/problem=6

pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "Sum Square Difference",
        number: 6,
        solve: || core_solve(100),
    }
}

fn core_solve(limit: u64) -> u64 {
    // sum (1 + 2 + 3 + ...) = n (n + 1) / 2
    // sum (1 + 2 + 3 + ...) = ( n(n+1)/2 ) ^ 2

    // sum (1^2 + 2^2 + 3^2 + ...) = n (n + 1) (2n + 1) / 3

    // so our answer is ( n(n+1)/2 ) ^ 2 -  n (n + 1) (2n + 1) / 3
    // which simplifies to the following:

    (limit) * (limit * limit - 1) * (3 * limit + 2) / 12
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(10), 2640)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 25164150)
    }
}
