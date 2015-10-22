extern crate rustc_serialize;
extern crate cryptopal;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Result;

use rustc_serialize::hex::FromHex;

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

    for line in lines {
        for c in 0u8..255u8 {
            let xored = xor(line, c);
            let score = english_score(&xored);
        }
        println!("{:?}", line);
    }
}
