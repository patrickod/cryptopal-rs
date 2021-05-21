extern crate cryptopal;
extern crate hex;

use cryptopal::oracle::{Oracle, RandomPrefixTargetSuffixEcbOracle};
use cryptopal::util;

fn main() {
    let oracle = RandomPrefixTargetSuffixEcbOracle::new();

    let block_size = util::calculate_oracle_block_size(&oracle).expect("block size");
    let total_payload_length =
        util::calculate_total_payload_length(&oracle).expect("payload length");
    let prefix_length = util::calculate_prefix_length(&oracle).expect("prefix length");
    let padding_to_block_edge = block_size - (prefix_length % block_size);

    let mut recovered_plaintext: Vec<u8> = Vec::new();
    let block_minus_one = vec![0; (padding_to_block_edge + block_size) - 1];
    let reference_block_end_ciphertexts = (0..block_size)
        .map(|x| oracle.encrypt(&block_minus_one[x..]))
        .collect::<Vec<Vec<u8>>>();

    for n in 0..(total_payload_length - prefix_length) {
        // index position of target block in the complete ciphertext
        let block_index = (padding_to_block_edge + prefix_length + n) / block_size;
        // range which provides a slice containing the target block within the ciphertext
        let block_window_range = (block_index * block_size)..((block_index + 1) * block_size);
        // byte index within the moving target block range
        let internal_byte_index = n % block_size;

        let matching_byte = (0..=255).find(|&u| {
            let candidate = [
                block_minus_one.clone(),
                recovered_plaintext.clone(),
                vec![u; 1],
            ]
            .concat();
            let ciphertext = oracle.encrypt(&candidate[internal_byte_index..]);

            reference_block_end_ciphertexts[internal_byte_index][block_window_range.clone()]
                == ciphertext[block_window_range.clone()]
        });
        match matching_byte {
            Some(b) => {
                recovered_plaintext.push(b);
            }
            None => {
                panic!("exhuasted guesses at target plaintext offset: {:02}", n);
            }
        }
    }
    assert_eq!(true, oracle.verify(&recovered_plaintext));
}
