extern crate cryptopal;
extern crate hex;
extern crate difference;

use cryptopal::oracle::{CbcTargetOracle, Oracle};
use cryptopal::xor;
use difference::Changeset;

fn main() {
    let oracle = CbcTargetOracle::new();
    let c1 = oracle.encrypt(";admin=true;1234".as_bytes());

    let mut t = vec![0u8; 15];
    t.push(b'X');
    let c2 = [
        &xor::xor(&c1[0..16], &t),
        &c1[16..]
    ].concat();

    let d1 = oracle.decrypt(&c1);
    let d2 = oracle.decrypt(&c2);

    d1.chunks(16).zip(d2.chunks(16)).for_each (|(ch1, ch2)| {
        println!("===");
        println!("{}", Changeset::new(&hex::encode(ch1), &hex::encode(ch2), ""));
    });

}
