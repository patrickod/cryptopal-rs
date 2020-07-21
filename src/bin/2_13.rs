extern crate cryptopal;

use std::str;

use cryptopal::oracle::ProfileOracle;
use cryptopal::profile::Profile;
use cryptopal::util;

fn main() {
    let oracle = ProfileOracle::new();
    println!(
        "block size: {}",
        util::determine_oracle_block_size(&oracle).unwrap()
    );
    println!(
        "payload length: {}",
        util::determine_payload_length(&oracle).unwrap()
    );
    println!("empty: {}", Profile::for_email(str::from_utf8(&[]).unwrap()).serialize());
    assert_eq!(
        "&uid=100&role=admin".len(),
        util::determine_payload_length(&oracle).unwrap()
    )
}
