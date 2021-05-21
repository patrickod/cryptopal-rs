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
    let payload_length = util::calculate_total_payload_length(&oracle).expect("bad payload");
    let prefix_length = util::calculate_prefix_length(&oracle).expect("bad oracle");
    let suffix_length = payload_length - prefix_length;
    let padding_to_block_edge = block_size - (prefix_length % block_size);

    // padding bytes necessary to align block edge at role=
    let role_assign_offset = (padding_to_block_edge + block_size + "user".len()) - suffix_length;

    // generate the first ciphertext w/ isolated 'user' component
    let c1 = oracle.encrypt(string_of_n(role_assign_offset).as_bytes());
    let c1_chunked: Vec<&[u8]> = c1.chunks(block_size).collect();

    // construct a new plaintext w/ (padding_to_block_edge|admin+pkcs) to
    // produce a ciphertext w/ isolated `admin` component to swap into C1
    let mut p2: Vec<u8> = Vec::new();
    p2.extend_from_slice(string_of_n(padding_to_block_edge).as_bytes());
    p2.extend(pkcs::pad("admin".as_bytes(), block_size));
    let c2 = oracle.encrypt(&p2);
    let c2_chunked: Vec<&[u8]> = c2.chunks(block_size).collect();

    // create our composite ciphertext using the first two blocks of the C1
    // ciphertext (designed to place the role= at the block edge)
    // append the C2 ciphertext block which contains the isolated `admin` literal
    let mut f: Vec<u8> = Vec::new();
    c1_chunked[0..2].iter().for_each(|c| f.extend_from_slice(c));
    f.extend(c2_chunked[1]);

    // decrypt & verify
    let d = oracle.decrypt(&f);
    println!("d: {}", str::from_utf8(&d).unwrap());
    assert_eq!(true, oracle.verify(&oracle.decrypt(&f)));
}
