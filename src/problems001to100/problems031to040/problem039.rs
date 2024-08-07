// https://projecteuler.net/problem=39

use integer_sqrt::IntegerSquareRoot;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Integer Right Triangles",
        number: 39,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let mut tris_by_perim = [0; 1001];
    let mut largest_perim = 0;

    for a in 1..tris_by_perim.len() {
        let a_squared = a * a;
        for b in a + 2..tris_by_perim.len() {
            let c_squared = a_squared + b * b;
            let c = c_squared.integer_sqrt();
            let current_perim = a + b + c;

            if current_perim >= tris_by_perim.len() {
                // all future b values will continue to yield a number too big
                // so we must reset a.
                break;
            }
            if c * c != c_squared {
                continue;
            }

            tris_by_perim[current_perim] += 1;

            if tris_by_perim[current_perim] > tris_by_perim[largest_perim] {
                largest_perim = current_perim
            }
        }
    }

    largest_perim as i64
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 840)
    }
}
