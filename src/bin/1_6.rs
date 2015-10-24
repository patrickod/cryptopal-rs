extern crate cryptopal;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Result,BufReader};

use rustc_serialize::base64::{FromBase64};

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

fn main () {
    let base64 = match load_data() {
        Ok(data) => data,
        Err(e) => { panic!("Unable to load data: {:?}", e); }
    };
    let data = match base64.from_base64() {
        Ok(d) => d,
        Err(e) => { panic!("Unable to decode base64 data: {:?}", e); }
    };

    println!("got data");
}
