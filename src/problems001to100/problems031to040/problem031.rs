// https://projecteuler.net/problem=31

use crate::euler_tools::collection_tools::CheckedGet;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Coin Sums",
        number: 31,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let available_coins: [i64; 7] = [1, 2, 5, 10, 20, 50, 100];
    const TARGET_VALUE: i64 = 200;

    let mut cache = [[0i64; TARGET_VALUE as usize + 1]; 7];
    for cache_row in cache.iter_mut() {
        (*cache_row)[0] = 1;
    }

    // Skip a bunch of bounds checks since the first row is simple.
    // For each value you want to make, there's only one way to make it if you only have pennies
    // of coin: all pennies
    for cache_value in cache[0].iter_mut() {
        *cache_value = 1;
    }

    for (coin_index, coin_value) in available_coins.iter().enumerate().skip(1) {
        for target_value in 0..=(TARGET_VALUE as usize) {
            cache[coin_index][target_value] = cache[coin_index - 1][target_value]
                + cache[coin_index]
                    .checked_get(target_value as i64 - coin_value)
                    .unwrap_or(&0);
        }
    }

    cache[cache.len() - 1][TARGET_VALUE as usize] + 1 // we haven't count the £2 coin on its own
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 73682)
    }
}
