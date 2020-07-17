extern crate rustc_serialize;
extern crate cryptopal;

use rustc_serialize::hex::ToHex;

use cryptopal::xor::repeating_character_xor;
use cryptopal::util::{english_score, load_data_lines};

struct Candidate {
    score: u32,
    line: Vec<u8>,
    character: u8,
}

fn main() {
    let lines = load_data_lines("./data/4.txt");
    let mut candidates: Vec<Candidate> = Vec::new();

    for line in lines {
        for c in 0u8..255u8 {
            let xored = repeating_character_xor(&line, c);
            let score = english_score(&xored);

            candidates.push(Candidate {
                score: score,
                line: line.clone(),
                character: c
            });
        }
    }

    // Sort by their scoroe ascending
    candidates.sort_by (|a, b| a.score.cmp(&b.score) );

    let last = match candidates.last() {
        Some(w) => w,
        None => { panic!("No winner!"); }
    };

    println!("winning character: {}", std::char::from_u32(last.character as u32).unwrap());
    println!("original line: {}", last.line.to_hex());
    println!("xored: {}", String::from_utf8(repeating_character_xor(&last.line, last.character)).unwrap());
}
