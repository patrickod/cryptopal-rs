extern crate cryptopal;
extern crate hex;

use std::iter::repeat;
use std::str;

use cryptopal::oracle::{Oracle, ProfileOracle};
use cryptopal::pkcs;
use cryptopal::util;

fn string_of_n(n: usize) -> String {
    repeat("B").take(n).collect::<String>()
}

fn main() {
    let oracle = ProfileOracle::new();

    let block_size = util::calculate_oracle_block_size(&oracle).expect("bad oracle");
    let payload_length = util::calculate_payload_length(&oracle).expect("bad payload");
    let prefix_length = util::calculate_prefix_length(&oracle).expect("bad oracle");
    let suffix_length = payload_length - prefix_length;
    let padding_to_block_edge = block_size - (prefix_length % block_size);

    // padding bytes to block edge + the amount necesassary to split the block
    // end at the beginning of `user`
    let offset = (padding_to_block_edge + block_size + "user".len()) - suffix_length;

    let c1 = oracle.encrypt(string_of_n(offset).as_bytes());
    let c1_chunked: Vec<&[u8]> = c1.chunks(block_size).collect();

    // construct a new plaintext with (padding_to_block_edge)+(admin+pkcs) to
    // produce our chosen ciphertext w/ isolated `admin` text for later
    // substitution
    let mut p2: Vec<u8> = Vec::new();
    p2.extend_from_slice(string_of_n(padding_to_block_edge).as_bytes());
    p2.extend(pkcs::pad("admin".as_bytes(), block_size));
    let c2 = oracle.encrypt(&p2);
    let c2_chunked: Vec<&[u8]> = c2.chunks(block_size).collect();

    // create our composite ciphertext using the first two "chunks" of the C1
    // ciphertext (designed to place the role= at the block edge)
    // append the C2 ciphertext chunk which contains the encrypted `admin` literal
    let mut f: Vec<u8> = Vec::new();
    c1_chunked[0..2].iter().for_each(|c| f.extend_from_slice(c));
    f.extend(c2_chunked[1]);

    println!(
        "{:?}",
        str::from_utf8(&oracle.decrypt(&f)).expect("bad utf8")
    );
}
