// https://projecteuler.net/problem=61

use crate::euler_tools::{
    figurate_numbers::{inverse_shape_lossy, shape},
    BoundedIterator,
};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Cyclical Figurate Numbers",
        number: 61,
        solve: || core_solve(6),
    }
}

fn core_solve(set_size: usize) -> i64 {
    let mut figurate_numbers: Vec<FigurateNumber> = Vec::new();

    for sides in 3..=(set_size + 2) {
        // this is to handle the case where 1000 is a figurate number.
        // Although I'm not aware of any specific reason it can't be one,
        // you can check that 1000 is not a triangular, ... , or octagonal number
        // Still, the only difference in terms of computation time is that
        // we add 1 at the end.
        let start_index = inverse_shape_lossy(999, sides) + 1;

        for figurate_number in
            BoundedIterator::new(10000, (start_index..).map(move |index| shape(index, sides)))
        {
            let mut new_figurate_number = FigurateNumber::new(
                ShapeType::from_sides(sides),
                figurate_number,
                figurate_numbers.len(),
            );
            for other_figurate_number in figurate_numbers.iter_mut() {
                new_figurate_number.try_add_valid_number(other_figurate_number);
                other_figurate_number.try_add_valid_number(&new_figurate_number);
            }
            figurate_numbers.push(new_figurate_number);
        }
    }

    for figurate_number in figurate_numbers.iter() {
        if let Some(sum) = figurate_number.try_find_cycle(set_size, &figurate_numbers) {
            return sum;
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ShapeType {
    Triangle = 1,
    Square = 2,
    Pentagon = 4,
    Hexagon = 8,
    Heptagon = 16,
    Octagon = 32,
}

impl ShapeType {
    pub fn from_sides(sides: usize) -> Self {
        match sides {
            3 => Self::Triangle,
            4 => ShapeType::Square,
            5 => ShapeType::Pentagon,
            6 => ShapeType::Hexagon,
            7 => ShapeType::Heptagon,
            8 => ShapeType::Octagon,
            _ => panic!("Unsupported shape: {sides} sides"),
        }
    }
}

#[derive(Debug)]
struct FigurateNumber {
    pub valid_next_numbers: Vec<usize>,
    pub shape_type: ShapeType,
    pub number: i64,
    pub index: usize,
}

impl FigurateNumber {
    pub fn new(shape_type: ShapeType, number: i64, index: usize) -> Self {
        FigurateNumber {
            valid_next_numbers: Vec::new(),
            shape_type,
            number,
            index,
        }
    }

    pub fn try_add_valid_number(&mut self, other_number: &FigurateNumber) {
        if self.shape_type == other_number.shape_type {
            return;
        }
        if self.number % 100 != other_number.number / 100 {
            return;
        }
        if self.number == other_number.number {
            return;
        }

        self.valid_next_numbers.push(other_number.index);
    }

    pub fn try_find_cycle(&self, cycle_size: usize, all_numbers: &[FigurateNumber]) -> Option<i64> {
        self.try_find_cycle_core(cycle_size, all_numbers, 0, &mut Vec::new(), self.index)
    }
    fn try_find_cycle_core(
        &self,
        depth: usize,
        all_numbers: &[FigurateNumber],
        shapes_used: usize,
        used_numbers: &mut Vec<i64>,
        initial_index: usize,
    ) -> Option<i64> {
        if used_numbers.contains(&self.number) || shapes_used & self.shape_type as usize != 0 {
            return None;
        }
        used_numbers.push(self.number);
        let shapes_used = shapes_used | self.shape_type as usize;

        if depth == 1 {
            return self
                .valid_next_numbers
                .contains(&initial_index)
                .then_some(self.number);
        }

        for next_candidate_index in self.valid_next_numbers.iter() {
            if let Some(sum) = all_numbers[*next_candidate_index].try_find_cycle_core(
                depth - 1,
                all_numbers,
                shapes_used,
                used_numbers,
                initial_index,
            ) {
                return Some(sum + self.number);
            }
        }

        used_numbers.pop();
        // we don't have to reset shapes_used because we shadowed the original variable

        None
    }
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
