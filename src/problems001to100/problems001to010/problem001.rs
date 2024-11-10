// https://projecteuler.net/problem=1

use crate::euler_tools::figurate_numbers::Triangle;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Multiples of 3 or 5",
        number: 1,
        solve: || core_solve(1000),
    }
}

fn core_solve(limit: i64) -> i64 {
    let limit = limit - 1;

    //        3 + 6 + 9 + ... + 3n
    // = 3 * (1 + 2 + 3 + ... +  n)
    // = 3 * ( n * (n + 1) / 2 )

    // we can't simplify further because we rely on the integer division truncation
    3 * Triangle::triangle(limit / 3) + 5 * Triangle::triangle(limit / 5)
        - 15 * Triangle::triangle(limit / 15)
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
