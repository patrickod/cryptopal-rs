extern crate cryptopal;
extern crate openssl;
extern crate rand;

use rand::prelude::*;

type Key = [u8; 16];

pub fn random_key() -> Key {
    let mut key: Key = [0u8; 16];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key.clone()
}

pub fn random_padding_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(5,11);
    let mut padding = vec![0u8; size];
    rng.fill_bytes(&mut padding);
    padding.clone()
}

pub fn main() {
    println!("{:?}", random_key());
    println!("{:?}", random_padding_bytes());
}

