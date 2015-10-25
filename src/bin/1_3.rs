extern crate rustc_serialize;
extern crate cryptopal;

use std::fmt;

use rustc_serialize::hex::FromHex;

use cryptopal::util::english_score;
use cryptopal::xor::repeating_character_xor;

struct Candidate {
    character: u8,
    score: u32
}

impl fmt::Debug for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", std::char::from_u32(self.character as u32).unwrap());
    }
}

pub fn main() {
    let cyphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    let candidate_range = 0u8..254u8;

    let mut candidates: Vec<Candidate> = candidate_range.map ( |c|
        Candidate {
            character: c,
            score: english_score(&repeating_character_xor(&cyphertext, c))
        }
    ).collect();

    candidates.sort_by (|a, b| a.score.cmp(&b.score) );

    println!("Winning character: {:?}", candidates.last().unwrap());
}
