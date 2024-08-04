use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Inv, One, PrimInt, Signed, Unsigned};

use crate::euler_tools::prime_finder::Primes;

#[allow(dead_code)]
pub trait MorePositiveConstants {
    fn two() -> Self;
    fn three() -> Self;
    fn four() -> Self;
    fn five() -> Self;
    fn six() -> Self;
    fn seven() -> Self;
    fn eight() -> Self;
    fn nine() -> Self;
    fn ten() -> Self;
}

impl<U: One + Add<Self, Output = Self>> MorePositiveConstants for U {
    fn two() -> Self {
        Self::one().add(Self::one())
    }

    fn three() -> Self {
        Self::two().add(Self::one())
    }

    fn four() -> Self {
        Self::three().add(Self::one())
    }

    fn five() -> Self {
        Self::four().add(Self::one())
    }

    fn six() -> Self {
        Self::five().add(Self::one())
    }

    fn seven() -> Self {
        Self::six().add(Self::one())
    }

    fn eight() -> Self {
        Self::seven().add(Self::one())
    }

    fn nine() -> Self {
        Self::eight().add(Self::one())
    }

    fn ten() -> Self {
        Self::nine().add(Self::one())
    }
}
mod test_constants {
    #[allow(unused_imports)]
    use super::MorePositiveConstants;

    #[test]
    fn test_two() {
        assert_eq!(u32::two(), 2u32);
    }
    #[test]
    fn test_three() {
        assert_eq!(u32::three(), 3u32);
    }
    #[test]
    fn test_four() {
        assert_eq!(u32::four(), 4u32);
    }
    #[test]
    fn test_five() {
        assert_eq!(u32::five(), 5u32);
    }
    #[test]
    fn test_six() {
        assert_eq!(u32::six(), 6u32);
    }
    #[test]
    fn test_seven() {
        assert_eq!(u32::seven(), 7u32);
    }
    #[test]
    fn test_eight() {
        assert_eq!(u32::eight(), 8u32);
    }
    #[test]
    fn test_nine() {
        assert_eq!(u32::nine(), 9u32);
    }
    #[test]
    fn test_ten() {
        assert_eq!(u32::ten(), 10u32);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ratio<N: PrimInt> {
    pub numerator: N,
    pub denominator: N,
}

impl<N: PrimInt> Ratio<N> {
    #[allow(dead_code)]
    pub fn new(numerator: N, denominator: N) -> Ratio<N> {
        Ratio {
            numerator,
            denominator,
        }
    }
    #[allow(dead_code)]
    pub fn new_int(numerator: N) -> Ratio<N> {
        Ratio {
            numerator,
            denominator: N::one(),
        }
    }
}

impl<N: PrimInt> One for Ratio<N> {
    fn one() -> Self {
        Ratio::new_int(N::one())
    }
}

// TODO I think I can clean up this implementation using macros, but I need to figure out
// how they work for trait implementations first

impl<O: PrimInt, N: PrimInt + Neg<Output = O> + Into<O>> Neg for Ratio<N> {
    type Output = Ratio<O>;

    fn neg(self) -> Self::Output {
        Ratio {
            numerator: -self.numerator,
            denominator: self.denominator.into(),
        }
    }
}
impl<N: PrimInt> Inv for Ratio<N> {
    type Output = Ratio<N>;

    fn inv(self) -> Self::Output {
        Ratio {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}

impl<M: PrimInt, O: PrimInt, N: PrimInt + Mul<M, Output = O>> Mul<Ratio<M>> for Ratio<N> {
    type Output = Ratio<O>;

    fn mul(self, rhs: Ratio<M>) -> Self::Output {
        Ratio {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<M: PrimInt, O: PrimInt, N: PrimInt + Mul<M, Output = O> + Into<O>> Mul<M> for Ratio<N> {
    type Output = Ratio<O>;

    fn mul(self, rhs: M) -> Self::Output {
        Ratio {
            numerator: self.numerator * rhs,
            denominator: self.denominator.into(),
        }
    }
}
impl<M, N: PrimInt> MulAssign<M> for Ratio<N>
where
    Ratio<N>: Mul<M, Output = Ratio<N>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let result = *self * rhs;
        self.numerator = result.numerator;
        self.denominator = result.denominator;
    }
}

impl<D: PrimInt, O: PrimInt, N: PrimInt + Mul<D, Output = O>> Div<Ratio<D>> for Ratio<N> {
    type Output = Ratio<O>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Ratio<D>) -> Self::Output {
        self * rhs.inv()
    }
}
impl<D: PrimInt, O: PrimInt, N: PrimInt + Mul<D, Output = O> + Into<O>> Div<D> for Ratio<N> {
    type Output = Ratio<O>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: D) -> Self::Output {
        Ratio {
            numerator: self.numerator.into(),
            denominator: self.denominator * rhs,
        }
    }
}
impl<D, N: PrimInt> DivAssign<D> for Ratio<N>
where
    Ratio<N>: Div<D, Output = Ratio<N>>,
{
    fn div_assign(&mut self, rhs: D) {
        let result = *self / rhs;
        self.numerator = result.numerator;
        self.denominator = result.denominator;
    }
}

impl<A: PrimInt, O: PrimInt, N: PrimInt + Add<O, Output = O> + Mul<A, Output = O>> Add<Ratio<A>>
    for Ratio<N>
{
    type Output = Ratio<O>;

    fn add(self, rhs: Ratio<A>) -> Self::Output {
        Ratio {
            numerator: self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<
        MO,
        A: PrimInt,
        AO: PrimInt,
        N: PrimInt + Mul<A, Output = MO> + Add<MO, Output = AO> + Into<AO>,
    > Add<A> for Ratio<N>
{
    type Output = Ratio<AO>;

    fn add(self, rhs: A) -> Self::Output {
        Ratio {
            numerator: self.numerator + self.denominator * rhs,
            denominator: self.denominator.into(),
        }
    }
}
impl<A, N: PrimInt> AddAssign<A> for Ratio<N>
where
    Ratio<N>: Add<A, Output = Ratio<N>>,
{
    fn add_assign(&mut self, rhs: A) {
        let result = *self + rhs;
        self.numerator = result.numerator;
        self.denominator = result.denominator;
    }
}

impl<S: PrimInt, O: PrimInt, N: PrimInt + Sub<O, Output = O> + Mul<S, Output = O>> Sub<Ratio<S>>
    for Ratio<N>
{
    type Output = Ratio<O>;

    fn sub(self, rhs: Ratio<S>) -> Self::Output {
        Ratio {
            numerator: self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<
        MO,
        S: PrimInt,
        SO: PrimInt,
        N: PrimInt + Mul<S, Output = MO> + Sub<MO, Output = SO> + Into<SO>,
    > Sub<S> for Ratio<N>
{
    type Output = Ratio<SO>;

    fn sub(self, rhs: S) -> Self::Output {
        Ratio {
            numerator: self.numerator - self.denominator * rhs,
            denominator: self.denominator.into(),
        }
    }
}
impl<S, N: PrimInt> SubAssign<S> for Ratio<N>
where
    Ratio<N>: Sub<S, Output = Ratio<N>>,
{
    fn sub_assign(&mut self, rhs: S) {
        let result = *self - rhs;
        self.numerator = result.numerator;
        self.denominator = result.denominator;
    }
}

pub trait PiecewiseAdd<Rhs> {
    type Output;

    #[allow(dead_code)]
    fn piecewise_add(self, rhs: Rhs) -> Self::Output;
}
impl<A: PrimInt, O: PrimInt, N: PrimInt + Add<A, Output = O>> PiecewiseAdd<Ratio<A>> for Ratio<N> {
    type Output = Ratio<O>;

    fn piecewise_add(self, rhs: Ratio<A>) -> Self::Output {
        Ratio {
            numerator: self.numerator + rhs.numerator,
            denominator: self.denominator + rhs.denominator,
        }
    }
}

pub trait Simplifiable {
    #[allow(dead_code)]
    fn simplify(self, primes: &Primes) -> Ratio<u64>;
}
pub trait SignedSimplifiable {
    #[allow(dead_code)]
    fn signed_simplify(self, primes: &Primes) -> Ratio<i64>;
}

impl<N: PrimInt + Into<u64> + Unsigned> Simplifiable for Ratio<N> {
    fn simplify(self, primes: &Primes) -> Ratio<u64> {
        let mut reduced_num: u64 = self.numerator.into();
        let mut reduced_denom: u64 = self.denominator.into();

        primes.find_gcd_and_reduce(&mut reduced_num, &mut reduced_denom);

        Ratio {
            numerator: reduced_num,
            denominator: reduced_denom,
        }
    }
}
impl<N: PrimInt + Into<i64> + Signed> SignedSimplifiable for Ratio<N> {
    fn signed_simplify(self, primes: &Primes) -> Ratio<i64> {
        let mut reduced_num: i64 = self.numerator.into();
        let mut reduced_denom: i64 = self.denominator.into();

        primes.find_gcd_and_reduce_signed(&mut reduced_num, &mut reduced_denom);

        Ratio {
            numerator: reduced_num,
            denominator: reduced_denom,
        }
    }
}

impl<N: PrimInt> PartialEq for Ratio<N> {
    fn eq(&self, other: &Self) -> bool {
        (self.numerator == other.numerator && self.denominator == other.denominator)
            || self.numerator * other.denominator == other.numerator * self.denominator
    }
}

mod test_ratio {
    #[allow(unused_imports)]
    use crate::euler_tools::additional_number_contansts::{
        Ratio, SignedSimplifiable, Simplifiable,
    };
    #[allow(unused_imports)]
    use crate::euler_tools::prime_finder::Primes;
    #[allow(unused_imports)]
    use num_traits::{Inv, One, PrimInt, Signed, Unsigned};
    #[allow(unused_imports)]
    use std::ops::{Add, Div, Mul, Neg, Sub};

    #[test]
    fn test_neg() {
        assert_eq!(-Ratio::new(1, 1), Ratio::new(-1, 1));
    }
    #[test]
    fn test_inv() {
        assert_eq!(Ratio::new(1, 2).inv(), Ratio::new(2, 1));
    }

    #[test]
    fn test_ratio_mul() {
        assert_eq!(Ratio::new(2, 3) * Ratio::new(4, 5), Ratio::new(8, 15));
    }
    #[test]
    fn test_const_mul() {
        assert_eq!(Ratio::new(2, 3) * 5, Ratio::new(10, 3));
    }

    #[test]
    fn test_ratio_div() {
        assert_eq!(Ratio::new(2, 3) / Ratio::new(4, 5), Ratio::new(10, 12));
    }
    #[test]
    fn test_const_div() {
        assert_eq!(Ratio::new(2, 3) / 5, Ratio::new(2, 15));
    }

    #[test]
    fn test_ratio_add() {
        assert_eq!(Ratio::new(2, 3) + Ratio::new(4, 5), Ratio::new(22, 15));
    }
    #[test]
    fn test_const_add() {
        assert_eq!(Ratio::new(2, 3) + 5, Ratio::new(17, 3));
    }

    #[test]
    fn test_ratio_sub() {
        assert_eq!(Ratio::new(2, 3) - Ratio::new(4, 5), Ratio::new(-2, 15));
    }
    #[test]
    fn test_const_sub() {
        assert_eq!(Ratio::new(2, 3) - 5, Ratio::new(-13, 3));
    }

    #[test]
    fn test_eq() {
        assert_eq!(Ratio::new(1, 2), Ratio::new(1, 2));
        assert!(Ratio::new(1, 2) == Ratio::new(5, 10));
    }

    #[test]
    fn test_unsigned_reduce() {
        let primes = Primes::find_primes(20);
        let orig_ratio = Ratio::new(5u64, 10u64);
        let reduced = orig_ratio.simplify(&primes);
        assert!(reduced == orig_ratio);
        assert_eq!(reduced.numerator, 1);
        assert_eq!(reduced.denominator, 2);
    }

    #[test]
    fn test_signed_reduce() {
        let primes = Primes::find_primes(20);
        let orig_ratio = Ratio::new(-5i64, 10i64);
        let reduced = orig_ratio.signed_simplify(&primes);
        assert!(reduced == orig_ratio);
        assert_eq!(reduced, Ratio::new(-1, 2));
    }
}
