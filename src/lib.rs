#[macro_use]
extern crate data_encoding_macro;
extern crate aes;
extern crate block_modes;
extern crate difference;
extern crate hex;
#[cfg(test)]
extern crate pretty_assertions;
extern crate rand;

pub mod oracle;
pub mod pkcs;
pub mod util;
pub mod xor;

pub const BLOCK_SIZE: usize = 16;
pub type AesKey = [u8; 16];

use rand::prelude::*;

pub fn random_key() -> AesKey {
    let mut key: AesKey = [0u8; 16];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key.clone()
}
