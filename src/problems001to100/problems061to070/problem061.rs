// https://projecteuler.net/problem=61

use crate::euler_tools;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Cyclical Figurate Numbers",
        number: 61,
        solve: || core_solve(6),
    }
}

enum ShapeType {
    Triange = 1,
    Square = 2,
    Pentagon = 4,
    Hexagon = 8,
    Heptagon = 16,
    Octagon = 32,
}

struct FigurateNumber<'a> {
    pub valid_next_numbers: Vec<&'a pntFigurateNumber>,
    pub shape_type: ShapeType,
    pub number: i64,
}

impl<'a> FigurateNumber<'a> {
    fn new(shape_type: ShapeType, number: i64) -> Self {
        return FigurateNumber {
            valid_next_numbers: Vec::new(),
            shape_type,
            number,
        };
    }

    fn try_add_valid_number(&mut self, other_number: &'a Figurate) {
        if self.shape_type == other_number.shape_type {
            return;
        }
        if self.number % 100 != other_number.number / 100 {
            return;
        }

        self.valid_next_numbers.push(other_number);
    }
}

fn core_solve(set_size: usize) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(3), 8128 + 2882 + 8281);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 28684);
    }
}
