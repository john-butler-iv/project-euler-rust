// https://projecteuler.net/problem=60

use crate::euler_tools::prime_finder::Primes;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Prime Pair Sets",
        number: 60,
        solve: || core_solve(5),
    }
}

fn core_solve(target_set_size: usize) -> i64 {
    let primes = Primes::find_primes(52000);
    let mut prime_pairs_sets = PrimePairsSets::initialize(&primes, target_set_size);

    loop {
        prime_pairs_sets.link_next_prime();
        if let Some(sum) = prime_pairs_sets.get_last_prime_set_sum() {
            return sum as i64;
        }
    }
}

/// Represents all associations for a single prime.
struct PrimeAssociations {
    prime_index: usize,
    paired_primes: Vec<usize>,
    /// self.valid_sets[size] contains a list of sets with this prime as the greatest.
    /// self.prime_index is included in the set.
    valid_sets: Vec<Vec<Vec<usize>>>,
}

impl PrimeAssociations {
    fn new(prime_index: usize, max_set_size: usize) -> PrimeAssociations {
        let mut valid_sets = vec![Vec::new(); max_set_size + 1];
        valid_sets[1].push(vec![prime_index]);
        PrimeAssociations {
            prime_index,
            paired_primes: Vec::new(),
            valid_sets,
        }
    }
    fn add_association(
        &mut self,
        other_prime_index: usize,
        other_assoation: &mut PrimeAssociations,
    ) {
        if self.prime_index < other_assoation.prime_index {
            other_assoation.add_association(self.prime_index, self);
            return;
        }
        self.paired_primes.push(other_prime_index);

        for other_valid_sets in other_assoation
            .valid_sets
            .iter()
            .take(self.valid_sets.len() - 1) // don't need to keep track of sets beyond the maximum
            .skip(1)
        // don't count size == 0
        {
            for other_valid_set in other_valid_sets {
                let mut matches = true;
                for required_pair_index in other_valid_set.iter().take(other_valid_set.len() - 1) {
                    if !self.is_paired(*required_pair_index) {
                        matches = false;
                    }
                }
                if !matches {
                    continue;
                }

                let mut new_set = other_valid_set.to_vec();
                new_set.push(self.prime_index);
                self.valid_sets[new_set.len()].push(new_set);
            }
        }
    }
    fn is_paired(&self, other_prime_index: usize) -> bool {
        // the list is sorted, but the list is small enought that we shouldn't binary search
        self.paired_primes.contains(&other_prime_index)
    }

    fn get_target_set_sum(&self, primes: &Primes) -> Option<u32> {
        self.valid_sets.last().and_then(|sets| {
            sets.first().map(|set| {
                set.iter()
                    .map(|prime_index| {
                        primes
                            .get_prime(*prime_index)
                            .expect("primes in this struct should be valided before inserting")
                    })
                    .sum()
            })
        })
    }
}

struct PrimePairsSets<'a> {
    primes: &'a Primes,
    next_prime_index: usize,
    prime_associations: Vec<PrimeAssociations>,
    max_set_size: usize,
}

impl<'a> PrimePairsSets<'a> {
    fn initialize(primes: &'a Primes, max_set_size: usize) -> PrimePairsSets<'a> {
        PrimePairsSets {
            primes,
            next_prime_index: 0,
            prime_associations: Vec::new(),
            max_set_size,
        }
    }

    pub fn link_next_prime(&mut self) {
        let next_prime = if let Some(next_prime) = self.primes.get_prime(self.next_prime_index) {
            next_prime
        } else {
            return;
        };

        let mut next_associataions =
            PrimeAssociations::new(self.next_prime_index, self.max_set_size);

        for (old_prime_index, other_prime_pairs) in self.prime_associations.iter_mut().enumerate() {
            let old_prime = self
                .primes
                .get_prime(old_prime_index)
                .expect("already tracked prime has already been validated in previous iterations");

            if are_prime_pair(old_prime, next_prime, self.primes) {
                next_associataions.add_association(old_prime_index, other_prime_pairs);
            }
        }

        self.prime_associations.push(next_associataions);
        self.next_prime_index += 1;
    }

    pub fn get_last_prime_set_sum(&self) -> Option<u32> {
        self.prime_associations
            .last()
            .and_then(|association| association.get_target_set_sum(self.primes))
    }
}

fn are_prime_pair(prime1: u32, prime2: u32, primes: &Primes) -> bool {
    let mut concat1 = prime1.to_string();
    let mut concat2 = prime2.to_string();

    concat1 += &concat2;
    concat2 += &prime1.to_string();

    primes.is_prime(concat1.parse().expect("just constructed strings from u32s"))
        && primes.is_prime(concat2.parse().expect("just constructed strings from u32s"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(4), 792);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 26033);
    }
}
