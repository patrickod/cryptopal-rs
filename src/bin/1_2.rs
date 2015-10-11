extern crate rustc_serialize;

use std::ops::BitXor;

use rustc_serialize::hex::{FromHex,ToHex};

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    return a.iter().zip(b.iter()).map(|(x,y)| *x ^ *y).collect();
}


pub fn main() {
    let x1 = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let x2 = "686974207468652062756c6c277320657965".from_hex().unwrap();

    println!("{}", &xor(&x1, &x2).to_hex());
}
