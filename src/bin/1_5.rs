extern crate cryptopal;
extern crate hex;

use cryptopal::xor;

fn main() {
    let text =
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
    let key = "ICE".as_bytes();

    let xored = xor::repeating_xor(&text, &key);
    println!("{}", hex::encode(xored));
}
