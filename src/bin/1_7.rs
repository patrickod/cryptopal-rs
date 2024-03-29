extern crate base64;
extern crate cryptopal;
extern crate openssl;

use cryptopal::util::load_file_as_vec;
use openssl::symm::{Cipher, Crypter, Mode};

fn main() {
    let input = load_file_as_vec("./data/7.txt");
    let data = match base64::decode(input) {
        Ok(e) => e,
        Err(e) => {
            panic!("Unable to decode b64 data: {:?}", e);
        }
    };

    // initalize the key and openssl crypter
    let k = "YELLOW SUBMARINE".to_string().into_bytes();
    let mut c = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &k, None).unwrap();
    c.pad(false);

    // Decrypt and finish out last block
    let mut decrypted: Vec<u8> = vec![0; data.len() + 128];
    c.update(&data, decrypted.as_mut_slice()).unwrap();
    let _ = c.finalize(decrypted.as_mut_slice());

    println!("{}", String::from_utf8(decrypted).unwrap());
}
