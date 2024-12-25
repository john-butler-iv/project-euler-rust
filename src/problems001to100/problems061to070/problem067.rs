// https://projecteuler.net/problem=67

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Maximum Path Sum II",
        number: 67,
        solve: core_solve,
    }
}

const TRIANGLE_FILENAME: &str = "./src/problems001to100/problems061to070/problem067_triangle.txt";

fn core_solve() -> i64 {
    solve_triangle(read_triangle(TRIANGLE_FILENAME))
}

fn read_triangle(filename: &str) -> Vec<Vec<i64>> {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Err(_) => return vec![],
        Ok(file) => file,
    };

    let mut triangle: Vec<Vec<i64>> = Vec::new();

    let reader = BufReader::new(&file);
    for line in reader.lines() {
        triangle.push(
            line.unwrap()
                .split(' ')
                .map(|num_string: &str| str::parse(num_string).unwrap())
                .collect(),
        );
    }

    triangle
}

fn solve_triangle(triangle: Vec<Vec<i64>>) -> i64 {
    let mut prev_row = vec![0; triangle.len() + 1];
    let mut current_row: Vec<i64> = Vec::new();

    for tri_row in triangle.iter().rev() {
        for (col_index, tri_val) in tri_row.iter().enumerate() {
            current_row.push(std::cmp::max(prev_row[col_index], prev_row[col_index + 1]) + tri_val)
        }
        std::mem::swap(&mut prev_row, &mut current_row);
        current_row.clear();
    }

    prev_row[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(
            super::solve_triangle(vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3],]),
            23
        );
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 7273);
    }
}
