// https://projecteuler.net/problem=18

use std::cmp::max;

use crate::euler_tools;

#[allow(clippy::zero_prefixed_literal)]
pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Maximum Path Sum I",
        number: 18,
        solve: || {
            core_solve(vec![
                75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34,
                88, 02, 77, 73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40,
                80, 70, 33, 41, 48, 72, 33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52,
                97, 51, 14, 70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14,
                91, 43, 58, 50, 27, 29, 48, 63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31,
                04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23,
            ])
        },
    }
}

fn core_solve(triangle: Vec<u64>) -> u64 {
    let size = euler_tools::inverse_triange(triangle.len() as u64) as usize;
    assert_eq!(triangle.len() as u64, euler_tools::triangle(size as u64));

    let mut cache = vec![0; triangle.len()];

    // copy last row
    cache[triangle.len() - size..].clone_from_slice(&triangle[triangle.len() - size..]);

    for row_index in (0..size - 1).rev() {
        let first_row_index = euler_tools::triangle(row_index as u64) as usize;
        for pos_index in first_row_index..=first_row_index + row_index {
            cache[pos_index] = triangle[pos_index]
                + max(
                    cache[pos_index + row_index + 1], // next row is bigger than this one. need an extra offset
                    cache[pos_index + row_index + 2],
                );
        }
    }

    cache[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3]), 23)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 1074)
    }
}
