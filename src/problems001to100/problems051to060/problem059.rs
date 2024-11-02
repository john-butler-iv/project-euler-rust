// https://projecteuler.net/problem=59

use std::io::BufRead;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "XOR Decryption",
        number: 59,
        solve: core_solve,
    }
}

const CIPHER_PATH: &str = "./src/problems001to100/problems051to060/problem059_cipher.txt";

fn core_solve() -> i64 {
    let cipher = read_cipher(CIPHER_PATH);

    for key1 in b'a'..=b'z' {
        for key2 in b'a'..=b'z' {
            for key3 in b'a'..=b'z' {
                let keys = [key1, key2, key3];
                let mut plaintext: Vec<u8> = Vec::with_capacity(cipher.len());
                let mut is_english = false;
                let mut sum = 0;

                for (index, cipher_char) in cipher.iter().enumerate().take(4) {
                    let plaintext_char = cipher_char ^ keys[index % keys.len()];
                    plaintext.push(plaintext_char);
                    sum += plaintext_char as i64;
                }
                for (index, cipher_char) in cipher.iter().enumerate().skip(4) {
                    let plaintext_char = cipher_char ^ keys[index % keys.len()];
                    plaintext.push(plaintext_char);
                    sum += plaintext_char as i64;

                    if plaintext[plaintext.len() - 5..] == *b" the " {
                        is_english = true;
                    }
                }

                if is_english {
                    return sum;
                }
            }
        }
    }

    unreachable!()
}

fn read_cipher(path: &str) -> Vec<u8> {
    let path = Path::new(path);
    let file = match File::open(path) {
        Err(_) => return vec![],
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    reader
        .split(b',')
        .map(|maybe_bytes| {
            String::from_utf8(maybe_bytes.expect("valid text file"))
                .expect("cipher file is ascii")
                .parse::<u8>()
                .expect("cipher file is digits and commas only")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 129448);
    }
}
