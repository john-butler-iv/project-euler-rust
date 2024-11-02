// https://projecteuler.net/problem=53

use std::{cmp::min, mem::swap};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Combinatoric Selectors",
        number: 53,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let mut total = 0;

    let mut line = &mut [0usize; 102];
    line[0] = 1;
    let mut next_line = &mut [0usize; 102];

    for _ in 0..=100 {
        for number in line.iter() {
            if *number > 1_000_000usize {
                total += 1;
            }
            if *number == 0 {
                break;
            }
        }

        update_line(line, next_line);
        swap(&mut line, &mut next_line);
    }

    total
}

fn update_line(line: &[usize], new_line: &mut [usize]) {
    new_line[0] = 1;
    for (index, new_number) in new_line.iter_mut().enumerate().skip(1) {
        if line[index - 1] == 0 {
            return;
        }
        *new_number = min(line[index] + line[index - 1], 10_000_000);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        let mut line = &mut [0usize; 102];
        line[0] = 1;
        let mut next_line = &mut [0usize; 102];

        for _ in 0..23 {
            super::update_line(line, next_line);
            std::mem::swap(&mut line, &mut next_line);
        }

        assert_eq!(line[10], 1144066);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 4075)
    }
}
