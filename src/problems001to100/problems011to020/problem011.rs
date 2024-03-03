// https://projecteuler.net/problem=11

use strum::IntoEnumIterator;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Largest Product in a Grid",
        number: 11,
        solve: core_solve,
    }
}

#[allow(clippy::zero_prefixed_literal)]
const GRID: [u64; 400] = [
    08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08, 49, 49, 99, 40,
    17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00, 81, 49, 31, 73, 55, 79, 14, 29,
    93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65, 52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56,
    01, 32, 56, 71, 37, 02, 36, 91, 22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28,
    66, 33, 13, 80, 24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70, 67, 26, 20, 68,
    02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21, 24, 55, 58, 05, 66, 73, 99, 26,
    97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72, 21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14,
    00, 61, 33, 97, 34, 31, 33, 95, 78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14,
    09, 53, 56, 92, 16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
    86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58, 19, 80, 81, 68,
    05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40, 04, 52, 08, 83, 97, 35, 99, 16,
    07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66, 88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67,
    46, 55, 12, 32, 63, 93, 53, 69, 04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32,
    40, 62, 76, 36, 20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
    20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54, 01, 70, 54, 71,
    83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
];

const ROWS: usize = 20;
const COLS: usize = 20;

const RUN_LENGTH: usize = 4;

#[derive(Debug, strum_macros::EnumIter)]
enum Direction {
    Down,
    Right,
    DownLeft,
    DownRight,
}

fn multilply_direction(row: usize, col: usize, direction: Direction) -> Option<u64> {
    let offset = match direction {
        Direction::Down => {
            if row + RUN_LENGTH > ROWS {
                return None;
            }
            COLS
        }
        Direction::Right => {
            if col + RUN_LENGTH > COLS {
                return None;
            }
            1
        }
        Direction::DownLeft => {
            if row + RUN_LENGTH > ROWS || col < RUN_LENGTH - 1 {
                return None;
            }
            COLS - 1
        }
        Direction::DownRight => {
            if row + RUN_LENGTH > ROWS || col + RUN_LENGTH > COLS {
                return None;
            }
            COLS + 1
        }
    };
    let root_index = row * COLS + col;
    let mut product = 1;
    for i in 0..RUN_LENGTH {
        product *= GRID[root_index + i * offset];
    }
    Some(product)
}

fn core_solve() -> u64 {
    let mut max_product = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            for direction in Direction::iter() {
                if let Some(current_product) = multilply_direction(row, col, direction) {
                    max_product = std::cmp::max(max_product, current_product);
                }
            }
        }
    }

    max_product
}

#[cfg(test)]
mod tests {

    use super::Direction;

    #[test]
    fn toy_example() {
        assert_eq!(
            super::multilply_direction(6, 8, Direction::DownRight),
            Some(1788696)
        )
    }

    #[test]
    fn down_in_bounds() {
        for row in 0..=(super::ROWS - super::RUN_LENGTH) {
            for col in (super::RUN_LENGTH - 1)..(super::COLS - super::RUN_LENGTH) {
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::Down),
                    None
                );
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::DownRight),
                    None
                );
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::DownLeft),
                    None
                );
            }
        }

        for row in 0..=(super::ROWS - super::RUN_LENGTH) {
            for col in 0..=(super::RUN_LENGTH - 1) {
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::Down),
                    None
                );
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::DownRight),
                    None
                );
            }
        }

        for row in 0..=(super::ROWS - super::RUN_LENGTH) {
            for col in (super::COLS - super::RUN_LENGTH)..super::COLS {
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::Down),
                    None
                );
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::DownLeft),
                    None
                );
            }
        }
    }
    #[test]
    fn right_in_bounds() {
        for row in 0..=(super::ROWS - super::RUN_LENGTH) {
            for col in 0..=(super::COLS - super::RUN_LENGTH) {
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::Right),
                    None
                );
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::DownRight),
                    None
                );
            }
        }
        for row in (super::ROWS - super::RUN_LENGTH)..super::ROWS {
            for col in 0..=(super::COLS - super::RUN_LENGTH) {
                assert_ne!(
                    super::multilply_direction(row, col, super::Direction::Right),
                    None
                );
            }
        }
    }

    #[test]
    fn down_out_of_bounds() {
        for row in (super::ROWS - super::RUN_LENGTH + 1)..=super::ROWS {
            for col in 0..super::COLS {
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::Down),
                    None
                );
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::DownRight),
                    None
                );
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::DownLeft),
                    None
                );
            }
        }
    }
    #[test]
    fn right_out_of_bounds() {
        for row in 0..=super::ROWS {
            for col in (super::COLS - super::RUN_LENGTH + 1)..=super::COLS {
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::Right),
                    None
                );
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::DownRight),
                    None
                );
            }
        }
    }

    #[test]
    fn left_out_of_bounds() {
        for row in 0..=super::ROWS {
            for col in 0..(super::RUN_LENGTH - 1) {
                assert_eq!(
                    super::multilply_direction(row, col, super::Direction::DownLeft),
                    None
                );
            }
        }
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 70600674)
    }
}
