extern crate cryptopal;
extern crate hex;

use cryptopal::xor::xor;

pub fn main() {
    let x1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let x2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    println!("{}", hex::encode(&xor(&x1, &x2)));
}
