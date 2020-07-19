extern crate cryptopal;
extern crate rustc_serialize;

use cryptopal::oracle::UNKNOWN_SUFFIX_BYTES;
use cryptopal::oracle::{Oracle, UnknownSuffixEcbOracle};
use cryptopal::BLOCK_SIZE;
use rustc_serialize::hex::ToHex;

/// determine the length of the oracle's prefix||suffix by encrypting
/// iteratively larger plaintexts until we observe a BLOCK_SIZE change in
/// ciphertext size.
/// at this point we know: len(plaintext) + len(prefix||oracle) % BLOCK_SIZE == 0
/// pkcs7 padding adds a complete BLOCK_SIZE of padding in this case also which
/// we subtract to ascertain the prefix||suffix size
fn determine_payload_length<T: Oracle>(oracle: &T) -> Option<usize> {
    let initial_length = oracle.encrypt(&[]).len();
    for n in 1..20 {
        let length = oracle.encrypt(&vec![0; n]).len();
        if length != initial_length {
            return Some(initial_length - n - BLOCK_SIZE as usize);
        }
    }
    None
}

#[test]
fn test_determine_payload_length() {
    let oracle = UnknownSuffixEcbOracle::new();
    assert_eq!(
        determine_payload_length(&oracle).unwrap(),
        UNKNOWN_SUFFIX_BYTES.len()
    );
}

fn main() {
    let oracle = UnknownSuffixEcbOracle::new();

    println!("{}", determine_payload_length(&oracle).unwrap());
    println!("suffix bytes: {}", UNKNOWN_SUFFIX_BYTES.len());
    println!(
        "{}",
        oracle.encrypt(&vec![0; 1])[..BLOCK_SIZE.into()].to_hex()
    );
    println!(
        "{}",
        oracle.encrypt(&vec![1; 1])[..BLOCK_SIZE.into()].to_hex()
    );
}
