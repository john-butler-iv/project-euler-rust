use std::mem::swap;

use icu::locid::extensions::other;

// https://projecteuler.net/problem=24
use crate::euler_tools::{self, collection_tools::inplace_permute};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Lexicographic Permutations",
        number: 24,
        solve: core_solve,
    }
}

fn core_solve() -> u64 {
    // If you notice, when you swap the 2nd to last and 3rd to last values, you're actually
    // skipping an extra value, and if you swap the 3rd and 4th from the end and the last
    // values are sorted, you're skipping 6:
    // V W X Y Z - position 0
    // V W X Z Y - position 1 - 1st/2nd swapped = 1!
    // V W Y X Z - position 2 - 2nd/3rd swapped = 2!
    // V W Y Z X - position 3
    // V W Z X Y - position 4
    // V W Z Y X - position 5
    // V X W Y Z - position 6 - 3rd/4th swapped = 3!
    // V X W Z Y - position 7
    // [...]
    // V Z Y X W - position 23
    // V W X Y Z - position 24 - 4th/5th swapped = 4!
    // As you can see, I'm claiming the number of advanced indexes
    // I'm not sure how you'd arrive at the exact answer since you can't just swap adjacent
    // numbers of a non-sorted subsequence, but you could certainly skip ahead a bunch.

    // if s is the initial permutation, then swapping s.0 and s.1 will skip you ahead (s.len() - 1)!
    // at a high level it's because 1. it takes (s.len() - 1)! moves until s.0 swaps because you
    // have to go through all possible permutations of [s.1 ..] before s.0 will ever move, and
    // 2. at that point, only s.0 and s.1 will have swapped and nothing else will have moved.

    let target_permutation_index = 1_000_000usize;
    let mut permutation = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut starting_permutation_index = 0;

    let fact_table = euler_tools::factorial_table(target_permutation_index);

    for (n, factorial) in fact_table.iter().enumerate().skip(1).rev() {
        if target_permutation_index - starting_permutation_index > *factorial {
            let swap_index = permutation.len() - n - 1;
            permutation.swap(swap_index, swap_index + 1);
            // The list will still be in order, so the same trick works, even though
            // n will already have been swapped
            starting_permutation_index += *factorial;
        }
    }

    // TODO I know there's got to be some way to apply the same logic again,
    // but I need to account for the fact that swapping s.0 and s.1 will take less than
    // (s.len() - 1)! iterations if the sub list to the right is not sorted. In essense,
    // you could think of it like it's mid way through the swap.

    for _ in starting_permutation_index..(1_000_000 - 1) {
        inplace_permute(&mut permutation);
    }

    permutation.iter().fold(0, |acc, &digit| acc * 10 + digit)
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 2783915460)
    }
}
