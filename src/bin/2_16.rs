extern crate cryptopal;
extern crate hex;

use std::str;

use cryptopal::oracle::{CbcTargetOracle, Oracle};
use cryptopal::util;
use cryptopal::pkcs;

fn main() {
    let oracle = CbcTargetOracle::new();
    let c1 = oracle.encrypt(";admin=true;1234".as_bytes());

    let block_size = util::calculate_oracle_block_size(&oracle).expect("block size");
    let total_payload_length =
        util::calculate_total_payload_length(&oracle).expect("payload length");
    let prefix_length = util::calculate_prefix_length(&oracle).expect("prefix length");
    let padding_to_block_edge = block_size - (prefix_length % block_size);

    println!(
        "{}",
        str::from_utf8(pkcs::strip(&oracle.decrypt(&c1)).unwrap()).unwrap()
    );
    println!(
        "block_size: {:02} total_payload_length: {:02} prefix_length: {:02} padding_to_block_edge: {:02}",
        block_size, total_payload_length, prefix_length, padding_to_block_edge
    )
}
