extern crate cryptopal;
extern crate rustc_serialize;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Result,BufReader};
use std::slice::Chunks;

use rustc_serialize::base64::FromBase64;
use rustc_serialize::hex::ToHex;
use itertools::Itertools;

use cryptopal::util::{english_score,hamming,transpose};
use cryptopal::xor::{repeating_character_xor,repeating_xor};

struct KeySize {
    size: u8,
    score: u32
}

struct Candidate {
    score: u32,
    character: u8,
}

fn load_data() -> Result<String> {
    let file = try!(File::open("./data/6.txt"));
    let mut base64: String = "".to_string();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = try!(line);
        base64.push_str(&line);
    }

    return Ok(base64);
}

fn normalized_edit_distance(data: &[u8], size: u8) -> u32 {
    let mut chunks = data.chunks(size as usize);
    let mut scores: Vec<u32> = Vec::new();
    let combinations = chunks.take(4).combinations();

    for pair in combinations {
        scores.push(hamming(&pair.0, &pair.1));
    }
    let sum = scores.iter().fold(0, (|sum, i| sum + i)) as u32;

    return (sum * 1000) / (size as u32);
}

fn compute_optimal_keysize(data: &[u8]) -> u8 {
    let mut keysizes: Vec<KeySize> = Vec::new();

    // Compute keysize distances
    for keysize in 2u8..40u8 {
        keysizes.push(KeySize {
            size: keysize,
            score: normalized_edit_distance(&data, keysize)
        });
    }
    // Sort by keysize ascending
    keysizes.sort_by (|a, b| a.score.cmp(&b.score) );

    let k = keysizes.first().unwrap();

    return k.size;
}

fn main () {
    let base64 = load_data().unwrap();
    let data = base64.from_base64().unwrap();
    let k = compute_optimal_keysize(&data);

    let chunks = data.chunks(k as usize);
    let transposed = transpose(&chunks, k);

    // Compute the single-character XOR for each transposed group.
    let mut key: Vec<u8> = Vec::new();
    for t in transposed.iter() {
        let mut candidates: Vec<Candidate> = Vec::new();
        for c in 0u8..255u8 {
            let xored = repeating_character_xor(&t, c);
            let score = english_score(&xored);

            candidates.push(Candidate {
                score: score,
                character: c
            });
        }
        candidates.sort_by (|a, b| a.score.cmp(&b.score) );
        key.push(candidates.last().unwrap().character.clone());
    }
    println!("{:?}", key);

    let plaintext = repeating_xor(&data, &key);
    println!("{}", String::from_utf8(plaintext).unwrap());
}
