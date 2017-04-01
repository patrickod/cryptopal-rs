extern crate cryptopal;
extern crate rustc_serialize;

use cryptopal::pkcs::pad;
use rustc_serialize::hex::ToHex;

fn main() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let padded = pad(input, 20);
    println!("{:?}", padded.to_hex());
}
