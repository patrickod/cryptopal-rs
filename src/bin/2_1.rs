extern crate cryptopal;

use cryptopal::pkcs::pad;

fn main() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let padded = pad(input, 20);
    println!("{:?}", padded);
}
