// https://projecteuler.net/problem=24
use crate::euler_tools::{self, collection_tools::inplace_permute};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Lexicographic Permutations",
        number: 24,
        solve: || core_solve(10, 1_000_000),
    }
}

fn find_longest_valid_run(
    permutation: &[i64],
    fact_table: &[usize],
    largest_factorial: usize,
) -> usize {
    for i in (1..permutation.len()).rev() {
        if fact_table[permutation.len() - i] > largest_factorial {
            return permutation.len() - i - 1;
        }
        if permutation[i - 1] > permutation[i] {
            return permutation.len() - i;
        }
    }
    if fact_table[permutation.len() - 1] > largest_factorial {
        return permutation.len() - 2;
    }
    permutation.len() - 1
}

fn core_solve(digits: usize, target_permutation_index: usize) -> i64 {
    // The core principle of this approach is the following fact:
    // if the permutation ends with an increasing subsequence of length n,
    // if you reverse the subsequence, you can skip n! permutations
    //
    // Justification:
    // for a sequence of length n, there are n! distinct permutations of the elements, assuming
    // all elements are unique. That's because there's n elements for the first posisiton,
    // then for each of those choices, you can choose out of the remaining n-1 elements, and
    // so on until you get n * (n-1) * ... * 2 * 1 = n!
    // Because these sequences are sorted in lexicographic order, we know that if all elements
    // are in ascending order, that's the first permutation. If all elements are in descending
    // order, we know that it's the last permutation of that sequence. Therefore, if we do
    // ever encounter a subsequence that's strictly increasing, we can reverse it to move
    // from the first permutation to the last of that subsequence, and we know that it would
    // have taken us n! - 1 moves to get there (n! total, and we started at 1).
    // I don't really have any fancy tricks after that, so we'll still have to use the normal
    // permutation algorithm.
    //
    // I don't really have to prove this, but generally advancing to the next permutation
    // from a decreasing subsequence, will result in a mostly increasing subsequence, so we
    // actually end up with very few iterations (25 with the real input)
    //
    // empirical evidence:
    // W X Y Z - position 1 = 0! (did no reversing) = 1! ("reversed" last element)
    // W X Z Y - position 2 = 2! (reversed last 2 elements)
    // W Y X Z - position 3
    // W Y Z X - position 4
    // W Z X Y - position 5
    // W Z Y X - position 6 = 3! (reversed last 3 elements)
    // X W Y Z - position 7
    // X W Z Y - position 8
    // [...]
    // Y Z W X - position 23
    // Z Y X W - position 24 = 4! (reversed last 4 elements)

    let mut permutation: Vec<i64> = (0..digits as i64).collect();
    let mut permutation_index = 1;

    let fact_table = euler_tools::factorial_table(target_permutation_index);

    while permutation_index < target_permutation_index - 1 {
        let longest_run = find_longest_valid_run(
            &permutation,
            &fact_table,
            target_permutation_index - 1 - permutation_index,
        );

        if longest_run > 1 {
            let index = permutation.len() - longest_run;
            permutation[index..].reverse();
            permutation_index += fact_table[longest_run] - 1;
        }

        // I don't want to check for this extra iteration, so I shoot for one less than the
        // required index and always do an extra permutation at the end.
        inplace_permute(&mut permutation);
        permutation_index += 1;
    }

    if permutation_index < target_permutation_index {
        inplace_permute(&mut permutation);
    }

    permutation.iter().fold(0, |acc, &digit| acc * 10 + digit)
}

#[cfg(test)]
mod tests {
    use crate::problems001to100::problems021to030::problem024::core_solve;

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn toy_example() {
        assert_eq!(core_solve(3, 1), 012);
        assert_eq!(core_solve(3, 2), 021);
        assert_eq!(core_solve(3, 3), 102);
        assert_eq!(core_solve(3, 4), 120);
        assert_eq!(core_solve(3, 5), 201);
        assert_eq!(core_solve(3, 6), 210);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 2783915460)
    }
}
