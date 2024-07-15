use std::ops::Add;

use num_traits::One;

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
