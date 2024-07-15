// https://projecteuler.net/problem=22

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const NAMES_PATH: &str = "./src/problems001to100/problems021to030/problem022_names.txt";

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Name Scores",
        number: 22,
        solve: || core_solve(&mut read_names(NAMES_PATH)),
    }
}

fn read_names(path: &str) -> Vec<String> {
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
            String::from_utf8(bytes[1..(bytes.len() - 1)].to_vec()).unwrap_or_default()
        })
        .collect()
}

fn core_solve(names: &mut [String]) -> u64 {
    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            name.as_bytes()
                .iter()
                .map(|letter| (letter - b'A' + 1) as u64 * (index + 1) as u64)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        let mut names = vec![String::new(); 937];
        names.push(String::from("COLIN"));

        assert_eq!(super::core_solve(&mut names), 49714)
    }

    #[test]
    fn colin_letters() {
        assert_eq!(super::core_solve(&mut [String::from("COLIN")]), 53);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 871198282)
    }

    #[test]
    fn number_of_names_correct() {
        assert!(super::read_names(super::NAMES_PATH).len() > 5000)
    }
}
