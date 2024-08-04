// https://projecteuler.net/problem=3

use num_traits::One;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::euler_tools::{
    additional_number_contansts::{PiecewiseAdd, Ratio, Simplifiable},
    prime_finder::Primes,
};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Digit Cancelling Fractions",
        number: 33,
        solve: core_solve,
    }
}

fn iterate_fractions() -> FractionIterator {
    // the earliest position we want is :
    /*
    FractionIterator {
        reducible_position: ReducePosition::first(),
        reducible_digit: 1,
        non_reducible_ratio: Ratio::new(1, 2),
    }
     */
    // When we call next, this state will update and return that desired position
    FractionIterator {
        reducible_position: ReducePosition::last(),
        reducible_digit: 9,
        non_reducible_ratio: Ratio::new(1, 1),
    }
}

#[derive(Clone, Copy, FromRepr, EnumCount, PartialEq)]
enum ReducePosition {
    FirstFirst,
    FirstSecond,
    SecondFirst,
    SecondSecond,
}

impl ReducePosition {
    pub fn first() -> Self {
        ReducePosition::from_repr(0).expect("First enum variant is 0-index")
    }
    pub fn last() -> Self {
        ReducePosition::from_repr(ReducePosition::COUNT - 1)
            .expect("All discriminents are consecutive and 0-indexed")
    }
    pub fn advance(self) -> Self {
        ReducePosition::from_repr((self as usize + 1) % ReducePosition::COUNT)
            .expect("All discriminents are consecutive and 0-indexed")
    }
}

struct FractionIterator {
    reducible_position: ReducePosition,
    reducible_digit: u64,
    non_reducible_ratio: Ratio<u64>,
}

impl Iterator for FractionIterator {
    type Item = (Ratio<u64>, Ratio<u64>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut state_ready = false;
        state_ready = state_ready || self.update_check_position();
        state_ready = state_ready || self.update_check_reducible_digit();
        state_ready = state_ready || self.update_check_denominator();
        state_ready = state_ready || self.update_check_numerator();

        if state_ready {
            let (reduce_mult, non_reduce_mult) = get_ratio_multipliers(self.reducible_position);
            let reducible_part =
                Ratio::new(self.reducible_digit, self.reducible_digit) * reduce_mult;
            let non_reduc_part = self.non_reducible_ratio * non_reduce_mult;
            Some((
                reducible_part.piecewise_add(non_reduc_part),
                self.non_reducible_ratio,
            ))
        } else {
            None
        }
    }
}

fn update_check_digit(digit: &mut u64, reset_digit: u64) -> bool {
    *digit += 1;
    if *digit >= 10 {
        *digit = reset_digit;
        false
    } else {
        true
    }
}

fn get_ratio_multipliers(reducible_position: ReducePosition) -> (Ratio<u64>, Ratio<u64>) {
    match reducible_position {
        ReducePosition::FirstFirst => (Ratio::new(10, 10), Ratio::new(1, 1)),
        ReducePosition::FirstSecond => (Ratio::new(10, 1), Ratio::new(1, 10)),
        ReducePosition::SecondFirst => (Ratio::new(1, 10), Ratio::new(10, 1)),
        ReducePosition::SecondSecond => (Ratio::new(1, 1), Ratio::new(10, 10)),
    }
}

impl FractionIterator {
    fn update_check_position(&mut self) -> bool {
        self.reducible_position = self.reducible_position.advance();

        // As long as we don't wrap, we're good to use the rest of the state
        self.reducible_position != ReducePosition::first()
    }
    fn update_check_reducible_digit(&mut self) -> bool {
        update_check_digit(&mut self.reducible_digit, 1)
    }

    fn update_check_numerator(&mut self) -> bool {
        update_check_digit(&mut self.non_reducible_ratio.numerator, 1)
    }

    fn update_check_denominator(&mut self) -> bool {
        // The problem specifies we only care about fractions < 1.
        // If the digits don't reduce, then we don't really care about the
        // output and we can safely skip
        // If the digits do reduce, then we need numerator < denominator
        // <=> denominator >= numerator + 1
        // but if we have to wrap back around, the numerator will increment as well,
        // so we want to have denominator >= new numerator + 1 = (current numerator + 1) + 1
        update_check_digit(
            &mut self.non_reducible_ratio.denominator,
            self.non_reducible_ratio.numerator + 2,
        )
    }
}

fn core_solve() -> i64 {
    const TOTAL_DIGIT_CANCELLING_FRACTIONS: i32 = 4;
    let mut discovered_fractions = 0;
    let mut fraction_product = Ratio::<u64>::one();

    for (big_fraction, fake_fraction) in iterate_fractions() {
        if big_fraction == fake_fraction {
            fraction_product *= fake_fraction;

            discovered_fractions += 1;
            if discovered_fractions >= TOTAL_DIGIT_CANCELLING_FRACTIONS {
                break;
            }
        }
    }

    // We know all of the composite fractions simplify to 1-digit numbers.
    let primes = Primes::find_primes(10);
    fraction_product
        .simplify(&primes)
        .denominator
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 100)
    }
}
