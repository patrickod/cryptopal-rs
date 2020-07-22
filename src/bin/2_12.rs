extern crate cryptopal;

use cryptopal::oracle::{Oracle,UnknownSuffixEcbOracle};
use cryptopal::util::{determine_payload_length,determine_oracle_block_size};

fn main() {
    let oracle = UnknownSuffixEcbOracle::new();
    let block_size = determine_oracle_block_size(&oracle).unwrap();
    let mut recovered: Vec<u8> = Vec::new();


    let input = vec![0; block_size - 1];
    let reference_ciphertexts = (0..block_size)
        .map(|x| oracle.encrypt(&input[x..]))
        .collect::<Vec<Vec<u8>>>();

    for n in 0..determine_payload_length(&oracle).unwrap() {
        let block_index = n / block_size;
        let byte_index = n % block_size;

        for u in 0u8..=255 {
            let t = [input.clone(), recovered.clone(), vec![u; 1]].concat();
            let ciphertext = oracle.encrypt(&t[byte_index..]);
            let block_range = (block_index * block_size)..((block_index + 1) * block_size);

            if reference_ciphertexts[byte_index][block_range.clone()]
                == ciphertext[block_range.clone()]
            {
                recovered.push(u);
                break;
            }
        }
    }

    println!("{}", String::from_utf8(recovered).expect("bad UTF8"));
}
