fn core_solve(limit: u64) -> u64 {
    (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "Multiples of 3 or 5",
        number: 1,
        solve: || core_solve(1000),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::core_solve(10), 23)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 233168)
    }
}
