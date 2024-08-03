// https://projecteuler.net/problem=17

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Number Letter Counts",
        number: 17,
        solve: || core_solve(1000),
    }
}

fn core_solve(max_num: u16) -> i64 {
    let mut total_letters = 0;
    for number in 1..=max_num {
        total_letters += numbers_to_letters(number);
    }

    total_letters
}

pub fn numbers_to_letters(number: u16) -> i64 {
    if number == 0 {
        0 // if number == 0, it's because it's the remainder of a larger number, so don't count. I.e., twenty, not twenty-zero
    } else if number < 10 {
        match number {
            1 => 3, // one
            2 => 3, // two
            3 => 5, // three
            4 => 4, // four
            5 => 4, // five
            6 => 3, // six
            7 => 5, // seven
            8 => 5, // eight
            9 => 4, // nine
            _ => panic!(),
        }
    } else if number < 100 {
        match number / 10 {
            // 0 handled by above case
            1 => match number {
                10 => 3, // ten
                11 => 6, // eleven
                12 => 6, // twelve
                13 => 8, // thirteen
                14 => 8, // fourteen
                15 => 7, // fifteen
                16 => 7, // sixteen
                17 => 9, // seventeen
                18 => 8, // eighteen
                19 => 8, // nineteen
                _ => panic!(),
            },
            2 => numbers_to_letters(number % 10) + 6, // twenty ...
            3 => numbers_to_letters(number % 10) + 6, // thirty ...
            4 => numbers_to_letters(number % 10) + 5, // fourty ...
            5 => numbers_to_letters(number % 10) + 5, // fifty ...
            6 => numbers_to_letters(number % 10) + 5, // sixty ...
            7 => numbers_to_letters(number % 10) + 7, // seventy ...
            8 => numbers_to_letters(number % 10) + 6, // eighty ...
            9 => numbers_to_letters(number % 10) + 6, // ninety ...
            _ => panic!(),
        }
    }
    // in the hundreds
    else if number % 100 != 0 {
        numbers_to_letters(number / 100) + numbers_to_letters(number % 100) + 10
        // # hundred and ...
    } else if number == 1000 {
        11 // one thousand
    } else {
        numbers_to_letters(number / 100) + 7
        // # hundred
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_to_five() {
        assert_eq!(super::numbers_to_letters(1), 3);
        assert_eq!(super::numbers_to_letters(2), 3);
        assert_eq!(super::numbers_to_letters(3), 5);
        assert_eq!(super::numbers_to_letters(4), 4);
        assert_eq!(super::numbers_to_letters(5), 4);

        assert_eq!(super::core_solve(5), 19);
    }

    #[test]
    fn three_four_two() {
        assert_eq!(super::numbers_to_letters(342), 23)
    }

    #[test]
    fn one_one_five() {
        assert_eq!(super::numbers_to_letters(115), 20)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 21124)
    }
}
