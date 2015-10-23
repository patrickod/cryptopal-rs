extern crate cryptopal;
extern crate rustc_serialize;

use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;

use rustc_serialize::hex::ToHex;

use cryptopal::xor;


fn read_data() -> Result<Vec<Vec<u8>>> {
    let file = try!(File::open("./data/5.txt"));
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<u8>> = Vec::new();


    for line in reader.lines() {
        let line = try!(line);
        lines.push(line.bytes().collect());
    }

    return Ok(lines);
}

fn main() {
    let lines = match read_data() {
        Ok(lines) => lines,
        Err(e) => { panic!("Unable to load 5.txt: {:?}", e); }
    };
    let key = "ICE".as_bytes();

    for line in lines {
        let xored = xor::repeating_xor(&line, &key);
        let hex = xored.to_hex();
        println!("{}", String::from_utf8(line).unwrap());
        println!("{}", hex);
    }
}
