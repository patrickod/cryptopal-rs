extern crate cryptopal;

use cryptopal::oracle::{CbcTargetOracle, Oracle};

fn main() {
    let oracle = CbcTargetOracle::new();
    let mut c1 = oracle.encrypt(";admin=true;1234".as_bytes());

    c1[16] = c1[16] ^ (b'_' ^ b';');
    c1[22] = c1[22] ^ (b'_' ^ b'=');
    c1[27] = c1[27] ^ (b'_' ^ b';');
    assert_eq!(true, oracle.verify(&oracle.decrypt(&c1)));
}
