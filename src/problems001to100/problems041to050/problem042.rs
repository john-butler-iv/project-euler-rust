// https://projecteuler.net/problem=42

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const WORDS_PATH: &str = "./src/problems001to100/problems041to050/problem042_words.txt";

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Coded Triangle Numbers",
        number: 42,
        solve: core_solve,
    }
}

fn read_words(path: &str) -> Vec<Vec<u8>> {
    let path = Path::new(path);
    let file = match File::open(path) {
        Err(_) => return vec![],
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    reader
        .split(b',')
        .map(|maybe_bytes| {
            let bytes = maybe_bytes.unwrap_or_default();
            bytes[1..(bytes.len() - 1)].to_vec()
        })
        .collect()
}

fn core_solve() -> i64 {
    // the longest word is 14 letters long. The maximum letter value is 26
    let mut is_triangular_cache = [false; 14 * 26 + 1];
    let mut current_tri = 1;
    let mut current_tri_index = 1;
    while current_tri < is_triangular_cache.len() {
        is_triangular_cache[current_tri] = true;
        current_tri_index += 1;
        current_tri += current_tri_index;
    }
    let is_triangular_cache = is_triangular_cache;

    read_words(WORDS_PATH)
        .iter()
        .map(|bytes| bytes.iter().map(|byte| byte - b'A' + 1).sum::<u8>())
        .filter(|n| is_triangular_cache[*n as usize])
        .collect::<Vec<_>>()
        .len() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 162)
    }
}
