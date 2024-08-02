// https://projecteuler.net/problem=24
use crate::euler_tools::{self, collection_tools::inplace_permute};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Lexicographic Permutations",
        number: 24,
        solve: core_solve,
    }
}

fn find_longest_valid_run(
    permutation: &[u64],
    fact_table: &[usize],
    largest_factorial: usize,
) -> usize {
    for i in (1..permutation.len()).rev() {
        if permutation[i - 1] > permutation[i] {
            return permutation.len() - i;
        }
        if fact_table[permutation.len() - i] > largest_factorial {
            return permutation.len() - i - 1;
        }
    }
    if fact_table[permutation.len() - 1] > largest_factorial {
        return permutation.len() - 2;
    }
    permutation.len() - 1
}

fn core_solve() -> u64 {
    // The core principle of this approach is the following fact:
    // if the permutation ends with an increasing subsequence of length n,
    // you can swap the (n-1)th and nth and elements from the end to skip forward (n-1)!
    // The last element counts as the 1st element from the end.
    // TODO Actually if you reverse the entire sequence, you can skip n!
    //
    // Justification:
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
    let mut permutation_index = 0;

    let fact_table = euler_tools::factorial_table(target_permutation_index);

    while permutation_index <= target_permutation_index {
        let mut longest_run = find_longest_valid_run(
            &permutation,
            &fact_table,
            target_permutation_index - permutation_index,
        );

        // if you have a long subsequence, you
        while longest_run > 1 {
            let swap_index = permutation.len() - longest_run;
            permutation.swap(swap_index, swap_index + 1);
            permutation_index += fact_table[longest_run - 1];
            longest_run -= 1;
        }

        inplace_permute(&mut permutation);
        permutation_index += 1;
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
