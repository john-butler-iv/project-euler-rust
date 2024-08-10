// https://projecteuler.net/problem=45

use std::cmp;

use crate::euler_tools::{Hexagon, Pentagon, Triangle};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Triangular, Pentagonal, and Hexagonal",
        number: 45,
        solve: || core_solve(285, 165, 143),
    }
}

fn core_solve(starting_t: i64, starting_p: i64, starting_h: i64) -> i64 {
    let mut t_index = starting_t + 1;
    let mut triangle = Triangle::triangle(t_index);
    let mut p_index = starting_p + 1;
    let mut pentagon = Pentagon::pentagon(p_index);
    let mut h_index = starting_h;
    let mut hexagon = Hexagon::hexagon(h_index);

    while triangle != pentagon {
        hexagon += 4 * h_index + 1;
        h_index += 1;
        loop {
            match hexagon.cmp(&pentagon) {
                cmp::Ordering::Less => {
                    hexagon += 4 * h_index + 1;
                    h_index += 1;
                }
                cmp::Ordering::Greater => {
                    pentagon += 3 * p_index + 1;
                    p_index += 1;
                }
                cmp::Ordering::Equal => break,
            }
        }
        while triangle < hexagon {
            t_index += 1;
            triangle += t_index;
        }
    }

    triangle
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(1, 1, 1), 40755)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 1533776805)
    }
}
