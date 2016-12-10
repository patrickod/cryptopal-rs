extern crate openssl;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Result,BufReader};

use rustc_serialize::base64::FromBase64;
use openssl::symm::{Crypter,Cipher,Mode};

fn load_data(path: &str) -> Result<Vec<u8>> {
    let file = File::open(path).unwrap();
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
    let mut c = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &k, None).unwrap();
    // let c = Crypter::new(Type::AES_128_ECB);

    // c.init(Mode::Decrypt, &k, &[]);
    c.pad(false);

    // Decrypt and finish out last block
    let mut decrypted: Vec<u8> = vec![0; data.len() + 128];
    c.update(&data, decrypted.as_mut_slice()).unwrap();
    c.finalize(decrypted.as_mut_slice());

    println!("{}", String::from_utf8(decrypted).unwrap());
}
