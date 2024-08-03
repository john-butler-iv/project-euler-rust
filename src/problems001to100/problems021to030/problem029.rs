// https://projecteuler.net/problem=29

use std::collections::HashSet;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Distinct Powers",
        number: 29,
        solve: || core_solve(100),
    }
}

fn core_solve(limit: u128) -> i64 {
    // this is really not ideal, but what're ya gonna do.
    let mut set: HashSet<String> = HashSet::new();

    for a in 2..=limit {
        for b in 2..=limit as u32 {
            set.insert((a as f64).powf(b as f64).to_string());
        }
    }

    set.len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(5), 15)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 9183)
    }
}
