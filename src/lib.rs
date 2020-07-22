#[macro_use]
extern crate data_encoding_macro;
extern crate aes;
extern crate rand;
extern crate block_modes;
extern crate hex;

pub mod xor;
pub mod util;
pub mod pkcs;
pub mod oracle;
pub mod profile;

pub const BLOCK_SIZE: usize = 16;
pub type AesKey = [u8; 16];

use rand::prelude::*;

pub fn random_key() -> AesKey {
    let mut key: AesKey = [0u8; 16];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key.clone()
}
