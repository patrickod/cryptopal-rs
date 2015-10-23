extern crate cryptopal;
extern crate rustc_serialize;

use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;

use rustc_serialize::hex::ToHex;

use cryptopal::xor;

fn main() {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
    let key = "ICE".as_bytes();

    let xored = xor::repeating_xor(&text, &key);
    let hex = xored.to_hex();
    println!("{}", hex);
}
