// https://projecteuler.net/problem=38

use crate::euler_tools::IsPandigital;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Pandigital Multiples",
        number: 38,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    for n in (1000_i64..10_000).rev() {
        if i64::are_combined_pandigital(&[n, n * 2]) {
            return n * 100000 + n * 2;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::euler_tools::IsPandigital;
    #[test]
    fn toy_example() {
        assert!(i64::are_combined_pandigital(&[192, 384, 576]));
        assert!(192384576.is_pandigital());
        assert!(i64::are_combined_pandigital(&[9, 18, 27, 36, 45]));
        assert!(918273645.is_pandigital());
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 932718654)
    }
}
