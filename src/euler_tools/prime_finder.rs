use std::cmp::max;

use integer_sqrt::IntegerSquareRoot;

use crate::euler_tools::BoundedRefIterator;

#[derive(Debug)]
pub struct Primes {
    prime_table: Vec<bool>,
    primes: Vec<u32>,
    limit: u32,
}

#[allow(dead_code)]
impl Primes {
    /// Finds the specified prime, starting with get_prime(0) -> Some(2)
    /// Returns None if requesting a prime beyond what has been computed
    pub fn get_prime(&self, prime_index: usize) -> Option<u32> {
        self.primes.get(prime_index).copied()
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

        // sqrt x is much less than x/log x ~= number of primes under x,
        // so we won't reserve extra capacity, but it's still better than nothing
        let mut primes: Vec<u32> = Vec::with_capacity(sqrt_limit);

        // If a number is not prime, it is guarunteed to have a divisor <= its square root
        for n in 2..=sqrt_limit {
            if !prime_table[n] {
                continue;
            }

            primes.push(n as u32);

            for n_multiple in (2 * n..limit).step_by(n) {
                prime_table[n_multiple] = false;
            }
        }

        let basic_start = sqrt_limit + 1;
        let next_odd_start = basic_start | 1;
        for i in (next_odd_start..limit).step_by(2) {
            if prime_table[i] {
                primes.push(i as u32);
            }
        }

        Primes {
            limit: limit as u32,
            primes,
            prime_table,
        }
    }

    pub fn is_prime_basic(&self, n: u32) -> bool {
        if n < self.limit {
            self.prime_table[n as usize]
        } else {
            panic!("Tried to find a prime beyond what has been computed")
        }
    }

    pub fn is_prime(&self, n: u32) -> bool {
        if n < self.limit {
            return self.prime_table[n as usize];
        } else if n < self.limit * self.limit {
            for prime in self.prime_iterator() {
                if n % prime == 0 {
                    return false;
                }
            }
            return true;
        }
        panic!("Tried to find a prime beyond what has been computed")
    }

    /// Iterates through all primes less than the limit
    pub fn bounded_prime_iterator(&self, limit: u32) -> BoundedRefIterator<u32> {
        BoundedRefIterator::new(limit, self.primes.iter())
    }

    pub fn prime_iterator(&self) -> std::slice::Iter<u32> {
        self.primes.iter()
    }

    pub fn prime_factorize(&self, n: u64) -> Vec<u64> {
        if n < self.limit as u64 && self.is_prime(n as u32) || n == 0 || n == 1 {
            return vec![n];
        }

        let mut factors: Vec<u64> = Vec::new();

        let mut n = n;
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

    pub fn unique_prime_factorize(&self, n: u64) -> Vec<u64> {
        if n < self.limit as u64 && self.is_prime(n as u32) || n == 0 || n == 1 {
            return vec![n];
        }

        let mut factors: Vec<u64> = Vec::new();

        let mut n = n;
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

    pub fn all_factors(&self, n: u64) -> Vec<u64> {
        if n < self.limit as u64 && self.is_prime(n as u32) {
            return vec![1, n];
        }
        if n < 2 {
            return vec![n];
        }

        let mut lower_factors = vec![1];
        let mut upper_factors = vec![n];

        let sqrt = n.integer_sqrt();

        for i in 2..sqrt {
            if n % i == 0 {
                lower_factors.push(i);
                upper_factors.push(n / i);
            }
        }

        if sqrt * sqrt == n {
            lower_factors.push(sqrt);
        } else if n % sqrt == 0 {
            lower_factors.push(sqrt);
            upper_factors.push(n / sqrt);
        }

        lower_factors.extend(upper_factors.iter().rev());
        lower_factors
    }

    pub fn divisors(&self, n: u64) -> usize {
        if n == 0 {
            return 0;
        }
        if n == 1 {
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

    pub fn sigma(&self, n: u64) -> u64 {
        self.all_factors(n).iter().sum()
    }

    fn sigma_cmp(&self, n: u64) -> std::cmp::Ordering {
        self.sigma(n).cmp(&(2 * n))
    }

    pub fn is_deficient(&self, n: u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Less
    }

    pub fn is_perfect(&self, n: u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Equal
    }

    pub fn is_abundant(&self, n: u64) -> bool {
        self.sigma_cmp(n) == std::cmp::Ordering::Greater
    }

    pub fn gcd(&self, a: u64, b: u64) -> u64 {
        self.find_gcd_and_reduce(&mut a.to_owned(), &mut b.to_owned())
    }
    pub fn gcd_signed(&self, a: i64, b: i64) -> u64 {
        self.find_gcd_and_reduce_signed(&mut a.to_owned(), &mut b.to_owned())
    }

    pub fn find_gcd_and_reduce(&self, a: &mut u64, b: &mut u64) -> u64 {
        if *a == 1 || *b == 1 {
            return 1;
        }
        if *a == 0 || *a % *b == 0 {
            let gcd = *b;
            *a /= gcd;
            *b = 1;
            return gcd;
        }
        if *b == 0 || *b % *a == 0 {
            let gcd = *a;
            *b /= gcd;
            *a = 1;
            return gcd;
        }

        // TODO I should actually verify that this approach works better finding both factors
        // and comparing the lists
        let mut gcd = 1;
        for a_factor in self.prime_factorize(*a) {
            if *b % a_factor == 0 {
                *b /= a_factor;
                gcd *= a_factor;
            }
        }

        *a /= gcd;

        gcd
    }

    pub fn find_gcd_and_reduce_signed(&self, a: &mut i64, b: &mut i64) -> u64 {
        let a_sign = a.signum() * b.signum();

        let mut unsigned_a = a.unsigned_abs();
        let mut unsigned_b = b.unsigned_abs();

        let gcd = self.find_gcd_and_reduce(&mut unsigned_a, &mut unsigned_b);

        *a = a_sign * (unsigned_a as i64);
        *b = unsigned_b as i64;

        gcd
    }
}

/// Generates pairs of values (p, q) where GCD(p, q) = 1 and p < q such that p, q < limit.
/// the pairs are not neccessarily in order.
#[derive(Debug)]
pub struct CoprimePairsIterator<I> {
    core_iter: CoreCoprimePairsIterator<I>,
    limit: I,
    is_done: bool,
}

#[derive(Debug)]
struct CoreCoprimePairsIterator<I> {
    current_pairs: Vec<Option<(I, I)>>,
    current_index: usize,
    next_pairs: Vec<Option<(I, I)>>,
    limit: I,
}

macro_rules! coprime_pairs_iterator_impl {
    ($($prim_type:ty),*) => { $(
        impl Iterator for CoprimePairsIterator<$prim_type>{
            type Item = ($prim_type, $prim_type);

            fn next(&mut self) -> Option<Self::Item>{
                if self.is_done {
                    return None;
                }

                let mut next_pair:Option<Self::Item> = self.core_iter.next();
                while next_pair.is_some_and(|next_pair|next_pair.1  >= self.limit) {
                    next_pair = self.core_iter.next();
                }

                if let Some(next_pair) = next_pair {
                    if next_pair == (self.limit - 2, self.limit - 1) {
                        self.is_done = true;
                    }
                }
                else {
                    self.is_done = true;
                }


                next_pair
            }
        }
        impl Iterator for CoreCoprimePairsIterator<$prim_type>{
            type Item = ($prim_type, $prim_type);

            fn next(&mut self) -> Option<Self::Item>{
                if self.current_index == self.current_pairs.len() - 1 {
                    self.reset_current_pairs();
                }

                while self.current_pairs[self.current_index+1].is_none() || self.current_pairs[self.current_index].is_none() {
                    if self.current_pairs[self.current_index + 1].is_none() {
                        self.current_index += 2;
                    } else if self.current_pairs[self.current_index].is_none() {
                        self.current_index += 1;
                    }
                    if self.current_index >= self.current_pairs.len() -1 {
                        return None;
                    }
                }

                let left = self.current_pairs[self.current_index].unwrap();
                let right = self.current_pairs[self.current_index + 1].unwrap();

                self.current_index += 1;

                let current_pair = (left.0 + right.0, left.1 + right.1);
                if current_pair.1 < self.limit{
                    self.next_pairs.push(Some(current_pair));
                } else if self.next_pairs.len() < 2 {
                    self.next_pairs.push(None);
                } else if let None = self.next_pairs[self.next_pairs.len() - 2]{
                    self.next_pairs.pop();
                } else {
                    self.next_pairs.push(None);
                }
                self.next_pairs.push(Some(right));

                Some(current_pair)
            }
        }
        impl CoprimePairsIterator<$prim_type>{
            pub fn new(limit: $prim_type) -> Self {
                CoprimePairsIterator {
                    core_iter: CoreCoprimePairsIterator::<$prim_type>::new(limit),
                    is_done: false,
                    limit: max(limit, 2),
                }
            }
        }

        impl CoreCoprimePairsIterator<$prim_type>{
            pub fn new(limit: $prim_type) -> Self {
                CoreCoprimePairsIterator{
                    current_pairs:vec![Some((0,1)), Some((1,1))],
                    current_index:0,
                    next_pairs:vec![Some((0,1))],
                    limit,
                }
            }

            fn reset_current_pairs(&mut self) {
                self.current_pairs = std::mem::replace(&mut self.next_pairs, vec![Some((0,1))]);
                self.current_index = 0;
                println!("{}",self.current_pairs.len());
            }
        }
    )* };
}
coprime_pairs_iterator_impl!(u8, u16, u32, u64, u128, usize);
coprime_pairs_iterator_impl!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::{CoprimePairsIterator, Primes};

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
            assert_eq!(primes.get_prime(i), Some(*p));
        }
    }

    #[test]
    fn prime_table_correct() {
        let primes = Primes::find_primes(100);
        for n in 0..primes.limit {
            assert_eq!(primes.is_prime(n), primes.primes.contains(&n))
        }
    }

    #[test]
    fn primes_factorized() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.prime_factorize(0), vec![0]);
        assert_eq!(primes.prime_factorize(1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(primes.prime_factorize(*p as u64), vec![*p as u64])
        }
    }

    #[test]
    fn primes_uniquely_factorized() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.unique_prime_factorize(0), vec![0]);
        assert_eq!(primes.unique_prime_factorize(1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(primes.unique_prime_factorize(*p as u64), vec![*p as u64])
        }
    }

    #[test]
    fn primes_all_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.all_factors(0), vec![0]);
        assert_eq!(primes.all_factors(1), vec![1]);
        for p in primes.primes.iter() {
            assert_eq!(primes.all_factors(*p as u64), vec![1, *p as u64])
        }
    }

    #[test]
    fn composites_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.prime_factorize(4), vec![2, 2]);
        assert_eq!(primes.prime_factorize(6), vec![2, 3]);
        assert_eq!(primes.prime_factorize(8), vec![2, 2, 2]);
        assert_eq!(primes.prime_factorize(9), vec![3, 3]);
        assert_eq!(primes.prime_factorize(10), vec![2, 5]);
        assert_eq!(primes.prime_factorize(12), vec![2, 2, 3]);
    }

    #[test]
    fn composites_uniquely_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.unique_prime_factorize(4), vec![2]);
        assert_eq!(primes.unique_prime_factorize(6), vec![2, 3]);
        assert_eq!(primes.unique_prime_factorize(8), vec![2]);
        assert_eq!(primes.unique_prime_factorize(9), vec![3]);
        assert_eq!(primes.unique_prime_factorize(10), vec![2, 5]);
        assert_eq!(primes.unique_prime_factorize(12), vec![2, 3]);
    }

    #[test]
    fn composites_all_factored() {
        let primes = Primes::find_primes(100);
        assert_eq!(primes.all_factors(4), vec![1, 2, 4]);
        assert_eq!(primes.all_factors(6), vec![1, 2, 3, 6]);
        assert_eq!(primes.all_factors(8), vec![1, 2, 4, 8]);
        assert_eq!(primes.all_factors(9), vec![1, 3, 9]);
        assert_eq!(primes.all_factors(10), vec![1, 2, 5, 10]);
        assert_eq!(primes.all_factors(12), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn limit_factored() {
        let primes_orig = Primes::find_primes(100);

        assert_eq!(
            primes_orig.prime_factorize((primes_orig.limit - 1) as u64),
            vec![3, 3, 11]
        );

        let last_prime = primes_orig
            .prime_iterator()
            .last()
            .expect("No prime found below 100");

        let primes = Primes::find_primes((last_prime + 1) as usize);

        assert_eq!(
            primes.prime_factorize(*last_prime as u64),
            vec![*last_prime as u64]
        );
    }
    #[test]
    fn limit_uniquely_factored() {
        let primes_orig = Primes::find_primes(100);

        assert_eq!(
            primes_orig.unique_prime_factorize((primes_orig.limit - 1) as u64),
            vec![3, 11]
        );

        let last_prime = primes_orig
            .prime_iterator()
            .last()
            .expect("No prime found below 100");

        let primes = Primes::find_primes((last_prime + 1) as usize);

        assert_eq!(
            primes.unique_prime_factorize(*last_prime as u64),
            vec![last_prime.to_owned() as u64]
        );
    }

    #[test]
    fn divisors_accurate() {
        let primes = Primes::find_primes(100);
        for n in 2..primes.limit as u64 {
            assert_eq!(
                primes.divisors(n),
                primes.all_factors(n).len(),
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
            assert_eq!(primes.is_deficient(n), defficient_numbers.contains(&n))
        }
    }

    #[test]
    fn first_hundred_are_perfect() {
        let primes = Primes::find_primes(100);
        let perfect_numbers = [6, 28];
        for n in 1..=100 {
            assert_eq!(primes.is_perfect(n), perfect_numbers.contains(&n))
        }
    }

    #[test]
    fn first_hundred_are_abundanat() {
        let primes = Primes::find_primes(100);
        let abundant = [
            12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72, 78, 80, 84, 88, 90, 96, 100,
        ];
        for n in 1..=100 {
            assert_eq!(primes.is_abundant(n), abundant.contains(&n))
        }
    }

    #[test]
    fn gcd() {
        let primes = Primes::find_primes(100);

        assert_eq!(primes.gcd(10, 20), 10);
        assert_eq!(primes.gcd_signed(10, -20), 10);

        let mut a = 10;
        let mut b = 20;
        assert_eq!(primes.find_gcd_and_reduce(&mut a, &mut b), 10);
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let mut a = -10;
        let mut b = 20;
        assert_eq!(primes.find_gcd_and_reduce_signed(&mut a, &mut b), 10);
        assert_eq!(a, -1);
        assert_eq!(b, 2);
    }

    #[test]
    fn coprime_pairs_generator() {
        let all_generated_coprime_pairs: Vec<_> = CoprimePairsIterator::<usize>::new(10).collect();

        let all_coprime_pairs = [
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 9),
            (2, 3),
            (2, 5),
            (2, 7),
            (2, 9),
            (3, 4),
            (3, 5),
            (3, 8),
            (3, 7),
            (4, 5),
            (4, 7),
            (4, 9),
            (5, 6),
            (5, 7),
            (5, 8),
            (5, 9),
            (6, 7),
            (7, 8),
            (7, 9),
            (8, 9),
        ];

        assert_eq!(all_coprime_pairs.len(), all_generated_coprime_pairs.len());
        for coprime_pair in all_coprime_pairs.iter() {
            assert!(all_generated_coprime_pairs.contains(coprime_pair));
        }
        for coprime_pair in all_generated_coprime_pairs.iter() {
            assert!(all_coprime_pairs.contains(coprime_pair));
        }
    }
}
