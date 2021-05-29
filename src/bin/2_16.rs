extern crate cryptopal;

use cryptopal::oracle::{CbcTargetOracle, Oracle};

fn main() {
    let oracle = CbcTargetOracle::new();
    let c1 = oracle.encrypt(";admin=true;1234".as_bytes());

    println!(
        "decrypted: {:?}",
        String::from_utf8_lossy(&oracle.decrypt(&c1))
    );
    println!("valid?: {}", oracle.verify(&oracle.decrypt(&c1)));
}
