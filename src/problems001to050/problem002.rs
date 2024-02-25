// https://projecteuler.net/problem=2

pub fn make() -> super::super::Problem {
    super::super::Problem {
        title: "Even Fibonacci Numbers",
        number: 2,
        solve: || core_solve(4_000_000),
    }
}

fn core_solve(limit: u32) -> u64 {
    u64::from(
        crate::euler_tools::BoundedIterator {
            // our fibonacci iterator generates 0, 1, 1, 2, 3, ...
            // but this problem expects 1, 2, 3, 5, ...
            // We need to skip the first two values
            iter: Box::new(
                crate::euler_tools::fibonacci_iterator::<u32>()
                    .skip(2)
                    .filter(|fib| fib % 2 == 0),
            ),
            bound: limit,
        }
        .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::core_solve(100), 44);
    }

    #[test]
    fn toy_fib_seq() {
        let mut iter = crate::euler_tools::fibonacci_iterator::<u32>().skip(2);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(34));
        assert_eq!(iter.next(), Some(55));
        assert_eq!(iter.next(), Some(89));
    }

    #[test]
    fn verify_result() {
        assert_eq!((super::make().solve)(), 4613732);
    }
}
