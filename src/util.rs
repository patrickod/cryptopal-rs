use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result,BufReader};
use std::slice::Chunks;

use rustc_serialize::hex::FromHex;

pub fn english_score(s: &[u8]) -> u32 {
    return s.iter().map (|&c| character_score(c)).fold(0u32, |sum, c| sum + c as u32);
}

fn character_score(c: u8) -> u32 {
    let c = match std::char::from_u32(c as u32) {
        Some(c) => c,
        None => { return 0; }
    };
    return match c {
        'z' => 74,
        'q' => 95,
        'x' => 150,
        'j' => 153,
        'k' => 772,
        'v' => 978,
        'b' => 1492,
        'p' => 1929,
        'y' => 1974,
        'g' => 2015,
        'f' => 2228,
        'w' => 2361,
        'm' => 2406,
        'u' => 2758,
        'c' => 2782,
        'l' => 4025,
        'd' => 4253,
        'r' => 5987,
        'h' => 6094,
        's' => 6327,
        'n' => 6749,
        'i' => 6966,
        'o' => 7507,
        'a' => 8167,
        't' => 9056,
        'e' => 12702,
        ' ' => 13000,
        _ => 0
    }
}

pub fn load_data(path: &str) -> Result<Vec<Vec<u8>>> {
    let file = try!(File::open(path));
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        lines.push(line.from_hex().unwrap());
    }
    return Ok(lines);
}

// calculate the hamming distance between two equal length slices of u8
pub fn hamming(a: &[u8], b: &[u8]) -> u32 {
    let pairs = a.iter().zip(b.iter());
    return pairs.map ( |(a,b)|
        (*a ^ *b).count_ones() as u32
    ).fold(0, ( |sum, i| sum + i )) as u32;
}

pub fn transpose(chunks: &Chunks<u8>, size: u8) -> Vec<Vec<u8>> {

    let mut results: Vec<Vec<u8>> = vec![Vec::new(); size as usize];

    for i in 0..size {
        for c in chunks.to_owned() {
            // The vec definitely exists
            let r = results.get_mut(i as usize).unwrap();

            // however the chunks aren't all guaranteed to be equal length
            match c.get(i as usize) {
                Some(v) => { r.push(v.clone()); },
                None => ()
            }
        }
    }

    return results;
}

#[test]
fn test_hamming() {
    let a = "this is a test".as_bytes();
    let b = "wokka wokka!!!".as_bytes();

    assert_eq!(hamming(&a, &b), 37);
}


#[test]
fn test_transpose() {
    let original = vec![1, 2, 3, 4, 5];
    let chunks = original.chunks(2);
    let transposed = transpose(&chunks, 2);

    let mut iter = transposed.iter();

    assert_eq!(iter.next().unwrap(), &[1, 3, 5]);
    assert_eq!(iter.next().unwrap(), &[2, 4]);
    assert_eq!(iter.next(), None);

}
