// https://projecteuler.net/problem=44

use crate::euler_tools::Pentagon;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Pentagon Numbers",
        number: 44,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let mut k = 3;
    loop {
        let p_k = Pentagon::pentagon(k);
        for j in (2..k).rev() {
            let p_j = Pentagon::pentagon(j);
            let pentagon_diff = p_k - p_j;
            if !Pentagon::is_pentagonal(pentagon_diff) {
                continue;
            }
            if !Pentagon::is_pentagonal(p_k + p_j) {
                continue;
            }
            return pentagon_diff;
        }
        k += 1
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 5482660)
    }
}
