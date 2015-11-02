extern crate openssl;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Result,BufReader};

use rustc_serialize::base64::FromBase64;
use openssl::crypto::symm::{Crypter,Type,Mode};

fn load_data(path: &str) -> Result<Vec<u8>> {
    let file = try!(File::open(path));
    let reader = BufReader::new(file);
    let mut out: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        out.extend(line.into_bytes());
    }

    return Ok(out);
}

fn main () {
    let base64 = match load_data("./data/7.txt") {
        Ok(d) => d,
        Err(e) => { panic!("Unable to load data: {:?}", e); }
    };
    let data = match base64.from_base64() {
        Ok(e) => e,
        Err(e) => { panic!("Unable to decode b64 data: {:?}", e); }
    };

    // initalize the key and openssl crypter
    let k = "YELLOW SUBMARINE".to_string().into_bytes();
    let c = Crypter::new(Type::AES_128_ECB);

    c.init(Mode::Decrypt, &k, &[]);
    c.pad(false);

    // Decrypt and finish out last block
    let mut decrypted = c.update(&data);
    decrypted.extend(c.finalize().into_iter());

    println!("{}", String::from_utf8(decrypted).unwrap());
}
