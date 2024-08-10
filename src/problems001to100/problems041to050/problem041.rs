// https://projecteuler.net/problem=41

use integer_sqrt::IntegerSquareRoot;

use crate::euler_tools::{
    collection_tools::{inplace_previous_permute, PermutationStatus},
    prime_finder::Primes,
    DigitIterator,
};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Pandigital Prime",
        number: 41,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let primes = Primes::find_primes(987654321.integer_sqrt() + 1);

    let mut digits = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut starting_digit_index = 0;
    while !primes.is_prime(DigitIterator::<u32>::combine_digits_big_endian(
        &digits[starting_digit_index..],
    )) {
        if inplace_previous_permute(&mut digits[starting_digit_index..])
            == PermutationStatus::Finished
        {
            // After the permutation is finished, the list is automatically put back into the
            // original increasing order
            starting_digit_index += 1;
        }
    }
    DigitIterator::<u32>::combine_digits_big_endian(&digits[starting_digit_index..]) as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 7652413)
    }
}
