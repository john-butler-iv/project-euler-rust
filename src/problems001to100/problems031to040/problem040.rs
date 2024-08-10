// https://projecteuler.net/problem=40

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Champernowne's Constant",
        number: 40,
        solve: || core_solve(&[1, 10, 100, 1000, 10_000, 100_000, 1_000_000]),
    }
}

fn core_solve(digit_locations: &[usize]) -> i64 {
    let mut digit_locations = digit_locations;
    let mut product = 1;

    let mut digit_len = 1;
    let mut digit_bound = 10;
    let mut distance_to_next_bound = 9;

    let mut current_number = 1;
    let mut current_digit_index = 1;
    while !digit_locations.is_empty() {
        let distance_to_next_desired_index = digit_locations[0] - current_digit_index;
        let numbers_to_skip = std::cmp::min(
            distance_to_next_desired_index / digit_len,
            distance_to_next_bound,
        );

        current_digit_index += numbers_to_skip * digit_len;
        current_number += numbers_to_skip;
        distance_to_next_bound -= numbers_to_skip;

        for current_digit in current_number
            .to_string()
            .into_bytes()
            .iter()
            .map(|digit| (digit - b'0') as i64)
        {
            if current_digit_index == digit_locations[0] {
                product *= current_digit;
                digit_locations = &digit_locations[1..];
                if digit_locations.is_empty() {
                    break;
                }
            }

            current_digit_index += 1;
        }

        current_number += 1;
        if current_number >= digit_bound {
            let new_digit_bound = digit_bound * 10;
            digit_len += 1;
            distance_to_next_bound = new_digit_bound - digit_bound - 1;
            digit_bound = new_digit_bound;
        } else {
            distance_to_next_bound -= 1;
        }
    }

    product
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(&[12]), 1)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 210)
    }
}
