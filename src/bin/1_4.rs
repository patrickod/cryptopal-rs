extern crate rustc_serialize;
extern crate cryptopal;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Result;

use rustc_serialize::hex::{FromHex,ToHex};

use cryptopal::xor::repeating_character_xor;
use cryptopal::util::english_score;

struct Candidate {
    score: i32,
    line: Vec<u8>,
    character: u8,
}


fn load_data() -> Result<Vec<Vec<u8>>> {
    let path = "./data/4.txt";
    let file = try!(File::open(path));
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        lines.push(line.from_hex().unwrap());
    }
    return Ok(lines);
}

fn main() {
    let lines = match load_data() {
        Ok(lines) => lines,
        Err(e) => { panic!("Unable to load data: {:?}", e); }
    };

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
