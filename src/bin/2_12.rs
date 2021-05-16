extern crate cryptopal;

use cryptopal::oracle::{Oracle, UnknownSuffixEcbOracle};
use cryptopal::util::{calculate_oracle_block_size, calculate_payload_length};

fn main() {
    let oracle = UnknownSuffixEcbOracle::new();
    let block_size = calculate_oracle_block_size(&oracle).unwrap();
    let mut recovered: Vec<u8> = Vec::new();

    let input = vec![0; block_size - 1];
    let reference_block_end_ciphertexts = (0..block_size)
        .map(|x| oracle.encrypt(&input[x..]))
        .collect::<Vec<Vec<u8>>>();

    for n in 0..calculate_payload_length(&oracle).unwrap() {
        let block_index = n / block_size;
        let byte_index = n % block_size;
        let block_window_range = (block_index * block_size)..((block_index + 1) * block_size);

        for u in 0u8..=255 {
            let t = [input.clone(), recovered.clone(), vec![u; 1]].concat();
            let ciphertext = oracle.encrypt(&t[byte_index..]);

            if reference_block_end_ciphertexts[byte_index][block_window_range.clone()]
                == ciphertext[block_window_range.clone()]
            {
                recovered.push(u);
                break;
            }
        }
    }

    println!("{}", String::from_utf8(recovered).expect("bad UTF8"));
}
