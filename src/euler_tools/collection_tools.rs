use std::cmp::Ordering;

pub trait CheckedGet<T> {
    fn checked_get<I: TryInto<usize>>(&self, index: I) -> Option<&T>;
}

impl<T> CheckedGet<T> for [T] {
    fn checked_get<I: TryInto<usize>>(&self, index: I) -> Option<&T> {
        let index: usize = index.try_into().ok()?;
        if index < self.len() {
            Some(&self[index])
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PermutationStatus {
    PermutationsRemaining,
    Finished,
}
pub fn inplace_permute<T: Ord>(permutation: &mut [T]) -> PermutationStatus {
    core_inplace_permute(permutation, Ordering::Less)
}

#[allow(dead_code)]
pub fn inplace_previous_permute<T: Ord>(permutation: &mut [T]) -> PermutationStatus {
    core_inplace_permute(permutation, Ordering::Greater)
}

fn core_inplace_permute<T: Ord>(
    permutation: &mut [T],
    final_ordering: Ordering,
) -> PermutationStatus {
    assert_ne!(final_ordering, Ordering::Equal);
    if permutation.is_empty() {
        return PermutationStatus::Finished;
    }

    let mut pivot = permutation.len() - 2;
    while permutation[pivot].cmp(&permutation[pivot + 1]) != final_ordering {
        pivot = match pivot.checked_sub(1) {
            Some(new_pivot) => new_pivot,
            None => {
                permutation.reverse();
                return PermutationStatus::Finished;
            }
        }
    }

    let mut lex_root = permutation.len() - 1;
    while lex_root > (pivot + 1) && permutation[pivot].cmp(&permutation[lex_root]) != final_ordering
    {
        lex_root -= 1;
    }

    permutation.swap(pivot, lex_root);
    permutation[pivot + 1..].reverse();

    PermutationStatus::PermutationsRemaining
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn inplace_permute_3_elems() {
        let mut permutation = [0, 1, 2];

        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [0, 2, 1]);
        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [1, 0, 2]);
        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [1, 2, 0]);
        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [2, 0, 1]);
        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [2, 1, 0]);
        assert_eq!(
            inplace_permute(&mut permutation),
            PermutationStatus::Finished
        );
        assert_eq!(permutation, [0, 1, 2]);
    }

    #[test]
    fn inplace_previous_permute_3_elems() {
        let mut permutation = [2, 1, 0];

        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [2, 0, 1]);
        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [1, 2, 0]);
        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [1, 0, 2]);
        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [0, 2, 1]);
        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::PermutationsRemaining
        );
        assert_eq!(permutation, [0, 1, 2]);
        assert_eq!(
            inplace_previous_permute(&mut permutation),
            PermutationStatus::Finished
        );
        assert_eq!(permutation, [2, 1, 0]);
    }

    #[test]
    fn inplace_permutes_cancel() {
        let original_permutation = [0, 1, 2];
        let mut permutation = original_permutation.to_owned();

        for total_iters in 1..=10 {
            for _ in 1..=total_iters {
                inplace_permute(&mut permutation);
            }
            for _ in 1..=total_iters {
                inplace_previous_permute(&mut permutation);
            }
            assert_eq!(permutation, original_permutation);

            for _ in 1..=total_iters {
                inplace_previous_permute(&mut permutation);
            }
            for _ in 1..=total_iters {
                inplace_permute(&mut permutation);
            }
            assert_eq!(permutation, original_permutation);
        }
    }
}
