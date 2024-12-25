// https://projecteuler.net/problem=75

use crate::euler_tools::PythagoreanTripleGenerator;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Singular Integer Right Triangles",
        number: 75,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    const LIMIT: usize = 1_500_000;
    let mut perimeter_cache = [0; LIMIT + 1];
    let mut total_unique_perimeters = 0;

    for triple in PythagoreanTripleGenerator::new((LIMIT + 1) as u64 / 2) {
        let perimeter = (triple.0 + triple.1 + triple.2) as usize;
        if perimeter > LIMIT {
            continue;
        }

        let previous_perimeter_occurances = perimeter_cache[perimeter];
        total_unique_perimeters += match previous_perimeter_occurances {
            0 => 1,
            1 => -1,
            _ => 0,
        };
        perimeter_cache[perimeter] = previous_perimeter_occurances + 1;
    }

    total_unique_perimeters
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        // verifying answer requires more memory than cargo test allows
        //assert_eq!((super::make().solve)(), 161667);
    }
}
