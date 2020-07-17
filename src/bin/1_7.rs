extern crate cryptopal;
extern crate openssl;
extern crate rustc_serialize;

use rustc_serialize::base64::FromBase64;
use openssl::symm::{Crypter,Cipher,Mode};
use cryptopal::util::load_data;

fn main () {
    let base64 = load_data("./data/7.txt");
    let data = match base64.from_base64() {
        Ok(e) => e,
        Err(e) => { panic!("Unable to decode b64 data: {:?}", e); }
    };

    // initalize the key and openssl crypter
    let k = "YELLOW SUBMARINE".to_string().into_bytes();
    let mut c = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &k, None).unwrap();
    // let c = Crypter::new(Type::AES_128_ECB);

    // c.init(Mode::Decrypt, &k, &[]);
    c.pad(false);

    // Decrypt and finish out last block
    let mut decrypted: Vec<u8> = vec![0; data.len() + 128];
    c.update(&data, decrypted.as_mut_slice()).unwrap();
    let _ = c.finalize(decrypted.as_mut_slice());

    println!("{}", String::from_utf8(decrypted).unwrap());
}
