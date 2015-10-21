extern crate rustc_serialize;

use std::fmt;

use rustc_serialize::hex::FromHex;

struct Candidate {
    character: u8,
    score: i32
}

impl fmt::Debug for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", std::char::from_u32(self.character as u32).unwrap());
    }
}


fn xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}

fn frequency_score(s: &[u8]) -> i32 {
    return s.iter().map (|&c| character_score(c)).fold(0i32, |sum, c| sum + c as i32);
}

fn character_score(c: u8) -> i32 {
    let character = match std::char::from_u32(c as u32) {
        Some(character) => character,
        None => { return 0; }
    };

    let mut score: i32 = 0;

    if c > 37 && c < 127 {
        score = score + 1
    }

    return match character {
        ' ' => score + 5,
        'e' => score + 5,
        't' => score + 5,
        'a' => score + 4,
        'o' => score + 4,
        'i' => score + 4,

        _ => score
    };
}

pub fn main() {
    let cyphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    let candidate_range = 0u8..254u8;

    let mut candidates: Vec<Candidate> = candidate_range.map ( |c|
        Candidate {
            character: c,
            score: frequency_score(&xor(&cyphertext, c))
        }
    ).collect();

    candidates.sort_by (|a, b| a.score.cmp(&b.score) );

    println!("Winning character: {:?}", candidates.last().unwrap());
}
