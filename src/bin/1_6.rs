extern crate base64;
extern crate cryptopal;
extern crate itertools;

use itertools::Itertools;

use cryptopal::util::{english_score, hamming, load_file_as_vec, transpose};
use cryptopal::xor::{repeating_character_xor, repeating_xor};

struct KeySize {
    size: u8,
    score: u32,
}

struct Candidate {
    score: u32,
    character: u8,
}

fn normalized_edit_distance(data: &[u8], size: u8) -> u32 {
    let chunks = data.chunks(size as usize);
    let mut scores: Vec<u32> = Vec::new();
    let combinations = chunks.take(4).combinations(2);

    for pair in combinations {
        scores.push(hamming(&pair[0], &pair[1]));
    }
    let sum = scores.iter().fold(0, |sum, i| sum + i) as u32;

    return (sum * 1000) / (size as u32);
}

fn compute_optimal_keysize(data: &[u8]) -> u8 {
    let mut keysizes: Vec<KeySize> = Vec::new();

    // Compute keysize distances
    for keysize in 2u8..40u8 {
        keysizes.push(KeySize {
            size: keysize,
            score: normalized_edit_distance(&data, keysize),
        });
    }
    // Sort by keysize ascending
    keysizes.sort_by(|a, b| a.score.cmp(&b.score));

    let k = keysizes.first().unwrap();

    return k.size;
}

fn main() {
    let input = String::from_utf8(load_file_as_vec("data/6.txt")).expect("bad UTF8");
    let data = base64::decode(input).unwrap();
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
                character: c,
            });
        }
        candidates.sort_by(|a, b| a.score.cmp(&b.score));
        key.push(candidates.last().unwrap().character.clone());
    }
    println!("{:?}", key);

    let plaintext = repeating_xor(&data, &key);
    println!("{}", String::from_utf8(plaintext).unwrap());
}
