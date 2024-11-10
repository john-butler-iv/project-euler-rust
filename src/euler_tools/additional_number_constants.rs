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
