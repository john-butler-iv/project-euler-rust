// https://projecteuler.net/problem=19

use icu::calendar::{types::IsoWeekday, Date};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Counting Sundays",
        number: 19,
        solve: core_solve,
    }
}

fn core_solve() -> u64 {
    let mut sunday_firsts = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if Date::try_new_iso_date(year, month, 1)
                .expect("If this isn't a valid date, the bounds needs to change")
                .day_of_week()
                == IsoWeekday::Sunday
            {
                sunday_firsts += 1;
            }
        }
    }

    sunday_firsts
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 171)
    }
}
