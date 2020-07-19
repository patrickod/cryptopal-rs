extern crate cryptopal;
extern crate rustc_serialize;

use cryptopal::oracle::{Oracle, UnknownSuffixEcbOracle};
use cryptopal::util::{determine_oracle_block_size_by_length,detect_duplicate_blocks};

fn main() {
    let oracle = UnknownSuffixEcbOracle::new();
    let block_size = determine_oracle_block_size_by_length(&oracle).unwrap();

    let uses_ecb = detect_duplicate_blocks(&oracle.encrypt(&vec![0; block_size * 2 + 1]));

    println!("Oracle Block Size: {}", block_size);
    println!("Oracle Uses ECB: {}", uses_ecb);
}
