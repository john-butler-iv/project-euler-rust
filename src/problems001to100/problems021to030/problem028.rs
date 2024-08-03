// https://projecteuler.net/problem=28

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Number Spiral Diagonals",
        number: 28,
        solve: || core_solve(1001),
    }
}

fn core_solve(grid_size: i64) -> i64 {
    let dia_len = grid_size / 2;
    dia_len * (16 * dia_len * dia_len + 30 * dia_len + 26) / 3 + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(5), 101)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 669171001)
    }
}
