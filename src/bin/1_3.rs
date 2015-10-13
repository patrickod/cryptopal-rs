extern crate rustc_serialize;

use std::ops::{Range,BitXor};
use std::ascii::AsciiExt;

use rustc_serialize::hex::FromHex;

fn xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}

fn character_score(s: &[u8]) -> u8 {
    return s.iter().take_while(|a| a.is_ascii() ).count() as u8;
}

pub fn main() {
    let cyphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    let candidates = (32..127);

    for candidate in candidates {
        let result = xor(&cyphertext, candidate);
        let candidate = vec![candidate];
        println!("{}, {:?}", String::from_utf8(candidate).unwrap(), String::from_utf8(result).unwrap());
    }
}
