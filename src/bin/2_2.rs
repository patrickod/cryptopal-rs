extern crate cryptopal;
extern crate openssl;
extern crate rustc_serialize;
extern crate crypto;
extern crate itertools;

use openssl::symm::{Crypter,Cipher,Mode};
use rustc_serialize::base64::FromBase64;

use cryptopal::util::load_data;
use cryptopal::xor::xor;

fn main() {
    let base64 = String::from_utf8(load_data("data/10.txt")).expect("bad UTF8");
    let data = base64.from_base64().expect("Unable to b64decode data");
    let key = "YELLOW SUBMARINE".as_bytes();

    let iv = vec![0x00; 16];
    let mut chunked_data = data.chunks(16 as usize);
    let first = chunked_data.next().expect("no data");

    let mut plaintext: Vec<u8> = xor(&iv, &decrypt_block(first, &key));
    chunked_data.map(|c| decrypt_block(c, &key)).zip(data.chunks(16 as usize)).for_each(
        |(a,b)| plaintext.extend(xor(&a, &b))
    );

    println!("{}", String::from_utf8(plaintext).expect("bad UTF8"));
}

pub fn decrypt_block(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut crypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    ).expect("Unable to initialize crypter");
    crypter.pad(false);

    // output needs to be at least >= input + block.len() otherwise we panic.
    // we only want the first block bytes later anyway.
    let mut output = vec![0; key.len() + block.len()];
    crypter.update(block, &mut output).expect("Unable to decrypt block");
    crypter.finalize(&mut output).expect("Unable to finalize decryption");
    output[..block.len()].to_owned()
}

pub fn encrypt_block(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut crypter = Crypter::new
}

#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    extern crate openssl;

    use rustc_serialize::hex::FromHex;
    use decrypt_block;
    use cryptopal::xor::xor;

    #[test]
    fn test_decrypt_block() {
        let block = "091230aade3eb330dbaa4358f88d2a6c".from_hex().expect("Unable to decode block");
        let iv = vec![0x00; 16];
        let key = "YELLOW SUBMARINE".as_bytes();
        let expected = "49276d206261636b20616e642049276d".from_hex().expect("Unable to decode expected result");

        let output = xor(&iv, decrypt_block(&block, &key).as_slice());
        assert_eq!(output[..16].to_owned(), expected);
    }
}
