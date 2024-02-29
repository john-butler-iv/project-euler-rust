use std::cmp::max;

use crate::euler_tools::BoundedRefIterator;

pub struct Primes {
    prime_table: Box<[bool]>,
    primes: Vec<u32>,
    limit: u32,
}

#[allow(dead_code)]
impl Primes {
    // mostly factoring out to share error message
    fn number_from_index(index: usize) -> u32 {
        u32::try_from(index).expect("usize should be at least 32 bits")
    }
    fn index_from_number(number: &u32) -> usize {
        usize::try_from(number.to_owned()).expect("usize should be at least 32 bits")
    }

    pub fn total_primes(&self) -> usize {
        self.primes.len()
    }

    pub fn find_primes(limit: usize) -> Self {
        let limit = max(limit, 1);
        let sqrt_limit = integer_sqrt::IntegerSquareRoot::integer_sqrt(&limit);

        let mut prime_table = vec![true; limit];
        prime_table[0] = false;
        prime_table[1] = false;

        let mut primes: Vec<u32> = Vec::new();

        for n in 2..=sqrt_limit {
            if !prime_table[n] {
                continue;
            }
            primes.push(Self::number_from_index(n));

            for n_multiple in (2 * n..limit).step_by(n) {
                prime_table[n_multiple] = false;
            }
        }

        for (n, is_prime) in prime_table.iter().enumerate().take(limit).skip(sqrt_limit) {
            if *is_prime {
                primes.push(Self::number_from_index(n));
            }
        }

        Primes {
            limit: Self::number_from_index(limit),
            primes,
            prime_table: prime_table.into_boxed_slice(),
        }
    }

    pub fn is_prime(&self, n: &u32) -> bool {
        if n < &self.limit {
            return self.prime_table[Self::index_from_number(n)];
        }
        todo!()
    }

    pub fn bounded_prime_iterator(&self, limit: u32) -> BoundedRefIterator<u32> {
        BoundedRefIterator::new(limit, self.primes.iter())
    }

    pub fn prime_iterator(&self) -> std::slice::Iter<u32> {
        self.primes.iter()
    }

    pub fn prime_factorize(&self, n: &u64) -> Vec<u64> {
        if n < &u64::from(self.limit)
            && self.is_prime(
                &(u32::try_from(n.to_owned()).expect("previously compared to less than a u32")),
            )
            || *n == 0
            || *n == 1
        {
            return vec![*n];
        }

        let mut factors: Vec<u64> = Vec::new();

        let mut n = n.to_owned();
        for p in self.prime_iterator() {
            let p = p.to_owned() as u64;
            while n % p == 0 {
                factors.push(p);
                n /= p;
            }
        }

        factors
    }

    pub fn unique_prime_factorize(&self, n: &u64) -> Vec<u64> {
        if n < &u64::from(self.limit)
            && self.is_prime(
                &(u32::try_from(n.to_owned()).expect("previously compared to less than a u32")),
            )
            || *n == 0
            || *n == 1
        {
            return vec![*n];
        }

        let mut factors: Vec<u64> = Vec::new();

        let mut n = n.to_owned();
        for p in self.prime_iterator() {
            let p = p.to_owned() as u64;
            if n % p == 0 {
                factors.push(p);
                loop {
                    n /= p;
                    if n % p != 0 {
                        break;
                    }
                }
            }
        }

        factors
    }
}

#[cfg(test)]
mod tests {
    use super::Primes;

    #[test]
    fn primes_generated() {
        let primes = Primes::find_primes(100);
        assert_eq!(
            primes.primes,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn prime_table_correct() {
        let primes = Primes::find_primes(100);
        for n in 0..primes.limit {
            assert_eq!(primes.is_prime(&n), primes.primes.contains(&n))
        }
    }

    #[test]
    fn primes_factorized() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.prime_factorize(&0), vec![0]);
        assert_eq!(primes.prime_factorize(&1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(
                primes.prime_factorize(&u64::from(p.to_owned())),
                vec![p.to_owned() as u64]
            )
        }
    }

    #[test]
    fn primes_uniquely_factorized() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.unique_prime_factorize(&0), vec![0]);
        assert_eq!(primes.unique_prime_factorize(&1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(
                primes.unique_prime_factorize(&u64::from(p.to_owned())),
                vec![p.to_owned() as u64]
            )
        }
    }

    #[test]
    fn composites_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.prime_factorize(&4), vec![2, 2]);
        assert_eq!(primes.prime_factorize(&6), vec![2, 3]);
        assert_eq!(primes.prime_factorize(&8), vec![2, 2, 2]);
        assert_eq!(primes.prime_factorize(&9), vec![3, 3]);
        assert_eq!(primes.prime_factorize(&10), vec![2, 5]);
        assert_eq!(primes.prime_factorize(&12), vec![2, 2, 3]);
    }

    #[test]
    fn composites_uniquely_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.unique_prime_factorize(&4), vec![2]);
        assert_eq!(primes.unique_prime_factorize(&6), vec![2, 3]);
        assert_eq!(primes.unique_prime_factorize(&8), vec![2]);
        assert_eq!(primes.unique_prime_factorize(&9), vec![3]);
        assert_eq!(primes.unique_prime_factorize(&10), vec![2, 5]);
        assert_eq!(primes.unique_prime_factorize(&12), vec![2, 3]);
    }

    #[test]
    fn limit_factored() {
        let primes_orig = Primes::find_primes(100);

        assert_eq!(
            primes_orig.prime_factorize(&u64::from(primes_orig.limit - 1)),
            vec![3, 3, 11]
        );

        let last_prime = primes_orig
            .prime_iterator()
            .last()
            .expect("No prime found below 100");

        let primes = Primes::find_primes(Primes::index_from_number(&(last_prime + 1)));

        assert_eq!(
            primes.prime_factorize(&u64::from(last_prime.to_owned())),
            vec![last_prime.to_owned() as u64]
        );
    }
    #[test]
    fn limit_uniquely_factored() {
        let primes_orig = Primes::find_primes(100);

        assert_eq!(
            primes_orig.unique_prime_factorize(&u64::from(primes_orig.limit - 1)),
            vec![3, 11]
        );

        let last_prime = primes_orig
            .prime_iterator()
            .last()
            .expect("No prime found below 100");

        let primes = Primes::find_primes(Primes::index_from_number(&(last_prime + 1)));

        assert_eq!(
            primes.unique_prime_factorize(&u64::from(last_prime.to_owned())),
            vec![last_prime.to_owned() as u64]
        );
    }

    #[test]
    fn bounded_prime_iterator() {
        let primes = Primes::find_primes(100);
        let bounded_primes: Vec<&u32> = primes.bounded_prime_iterator(50).collect();
        assert_eq!(
            bounded_primes,
            vec![&2, &3, &5, &7, &11, &13, &17, &19, &23, &29, &&31, &37, &41, &43, &47,]
        );
    }
}
