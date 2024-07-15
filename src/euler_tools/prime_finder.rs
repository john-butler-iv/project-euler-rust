use std::cmp::max;

use integer_sqrt::IntegerSquareRoot;

use crate::euler_tools::BoundedRefIterator;

#[derive(Debug)]
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

    pub fn get_prime(&self, prime: usize) -> Option<&u32> {
        self.primes.get(prime)
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

        for (n, is_prime) in prime_table
            .iter()
            .enumerate()
            .take(limit)
            // we've already added up to sqrt_limit. Because prime_table is 0-indexed, we need to skip one more
            .skip(sqrt_limit + 1)
        {
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
            if n == 1 {
                break;
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

                if n == 1 {
                    break;
                }
            }
        }

        factors
    }

    pub fn all_factors(&self, n: &u64) -> Vec<u64> {
        if n < &u64::from(self.limit)
            && self.is_prime(&u32::try_from(*n).expect("just checked less than u32"))
        {
            return vec![1, *n];
        }
        if *n < 2 {
            return vec![*n];
        }

        let mut lower_factors = vec![1];
        let mut upper_factors = vec![*n];

        let sqrt = n.integer_sqrt();

        for i in 2..sqrt {
            if n % i == 0 {
                lower_factors.push(i);
                upper_factors.push(n / i);
            }
        }

        if sqrt * sqrt == *n {
            lower_factors.push(sqrt);
        } else if n % sqrt == 0 {
            lower_factors.push(sqrt);
            upper_factors.push(n / sqrt);
        }

        lower_factors.extend(upper_factors.iter().rev());
        lower_factors
    }

    pub fn divisors(&self, n: &u64) -> usize {
        if *n == 0 {
            return 0;
        }
        if *n == 1 {
            return 1;
        }
        let factors = self.prime_factorize(n);
        let mut divisors: usize = 1;

        let mut factor_index = 0;
        while factor_index < factors.len() {
            for next_factor_index in factor_index + 1..=factors.len() {
                if next_factor_index < factors.len()
                    && factors[factor_index] == factors[next_factor_index]
                {
                    continue;
                }

                let num_factors = next_factor_index - factor_index + 1;
                divisors *= num_factors;
                factor_index = next_factor_index
            }
        }

        divisors
    }

    pub fn sigma(&self, n: &u64) -> u64 {
        self.all_factors(n).iter().sum()
    }

    fn sigma_cmp(&self, n: &u64) -> std::cmp::Ordering {
        self.sigma(n).cmp(&(2 * n))
    }

    pub fn is_deficient(&self, n: &u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Less
    }

    pub fn is_perfect(&self, n: &u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Equal
    }

    pub fn is_abundant(&self, n: &u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Greater
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
                83, 89, 97,
            ]
        );
    }

    #[test]
    fn get_prime() {
        let primes = Primes::find_primes(100);
        let known_primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        for (i, p) in known_primes.iter().enumerate() {
            assert_eq!(primes.get_prime(i), Some(p));
        }
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
    fn primes_all_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.all_factors(&0), vec![0]);
        assert_eq!(primes.all_factors(&1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(
                primes.all_factors(&u64::from(p.to_owned())),
                vec![1, p.to_owned() as u64]
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
    fn composites_all_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.all_factors(&4), vec![1, 2, 4]);
        assert_eq!(primes.all_factors(&6), vec![1, 2, 3, 6]);
        assert_eq!(primes.all_factors(&8), vec![1, 2, 4, 8]);
        assert_eq!(primes.all_factors(&9), vec![1, 3, 9]);
        assert_eq!(primes.all_factors(&10), vec![1, 2, 5, 10]);
        assert_eq!(primes.all_factors(&12), vec![1, 2, 3, 4, 6, 12]);
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
    fn divisors_accurate() {
        let primes = Primes::find_primes(100);
        for n in 2..primes.limit as u64 {
            assert_eq!(
                primes.divisors(&n),
                primes.all_factors(&n).len(),
                "finding divisors of {n}."
            );
        }
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

    #[test]
    fn first_hundred_are_deficient() {
        let primes = Primes::find_primes(100);
        let defficient_numbers = [
            1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 21, 22, 23, 25, 26, 27, 29, 31,
            32, 33, 34, 35, 37, 38, 39, 41, 43, 44, 45, 46, 47, 49, 50, 51, 52, 53, 55, 57, 58, 59,
            61, 62, 63, 64, 65, 67, 68, 69, 71, 73, 74, 75, 76, 77, 79, 81, 82, 83, 85, 86, 87, 89,
            91, 92, 93, 94, 95, 97, 98, 99,
        ];
        for n in 1..=100 {
            assert_eq!(primes.is_deficient(&n), defficient_numbers.contains(&n))
        }
    }

    #[test]
    fn first_hundred_are_perfect() {
        let primes = Primes::find_primes(100);
        let perfect_numbers = [6, 28];
        for n in 1..=100 {
            assert_eq!(primes.is_perfect(&n), perfect_numbers.contains(&n))
        }
    }

    #[test]
    fn first_hundred_are_abundanat() {
        let primes = Primes::find_primes(100);
        let abundant = [
            12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72, 78, 80, 84, 88, 90, 96, 100,
        ];
        for n in 1..=100 {
            assert_eq!(primes.is_abundant(&n), abundant.contains(&n))
        }
    }
}
