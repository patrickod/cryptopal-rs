extern crate cryptopal;

use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;

use cryptopal::xor;


fn read_data() -> Result<Vec<Vec<u8>>> {
    let file = try!(File::open("./data/5.txt"));
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<u8>> = Vec::new();


    for line in reader.lines() {
        let line = try!(line);
        lines.push(line);
    }

    return lines;
}

fn main() {
    let data = read_data();
    let key = "ICE".to_vec();

    for line in data {
        let xored = xor::repeating_xor(&line, &key);
        println!("Xored: {}", String::from_utf8(xored).unwrap());
    }
}
