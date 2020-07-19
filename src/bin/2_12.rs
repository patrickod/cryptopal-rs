extern crate cryptopal;
extern crate rustc_serialize;

use cryptopal::oracle::{Oracle, UnknownSuffixEcbOracle};

fn determine_oracle_block_size_by_length<T: Oracle>(oracle: &T) -> Option<usize> {
    let initial_length = oracle.encrypt(&vec![]).len();
    for n in 0..20 {
        let length = oracle.encrypt(&vec![0; n]).len();
        if length != initial_length {
            return Some(length - initial_length);
        }
    }
    None
}

fn main() {
    let oracle = UnknownSuffixEcbOracle::new();
    println!(
        "Oracle Block Length: {}",
        determine_oracle_block_size_by_length(&oracle).unwrap()
    );
}
