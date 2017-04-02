extern crate cryptopal;
extern crate openssl;
extern crate rustc_serialize;
extern crate crypto;

use openssl::symm::{Crypter,Cipher,Mode};
use rustc_serialize::base64::FromBase64;
use rustc_serialize::hex::ToHex;
use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use cryptopal::util::load_data_single_line;
use cryptopal::xor::fixed_xor;

fn main() {
    let base64 = load_data_single_line("data/10.txt").expect("Unable to load 10.txt");
    let data = base64.from_base64().expect("Unable to b64decode data");
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = vec![0x00; 16];

    let mut block_output = vec![0u8; 32];
    let mut plaintext: Vec<u8> = vec![];

    let mut chunks = data.chunks(16 as usize);
    let first = chunks.next().expect("Unable to read first 16 bytes from ciphertext");

    // decrypt_block(&iv, first, &key, block_output.as_mut_slice());
    // plaintext.extend(block_output[1..16].to_owned());

    // let second = chunks.next().expect("Unable to read second 16 bytes from ciphertext");

    // decrypt_block(
    //     &plaintext[(plaintext.len() - 16)..],
    //     second,
    //     &key,
    //     block_output.as_mut_slice()
    // );

    // plaintext.extend(block_output[..16].to_owned());

    // println!("Output: {}", String::from_utf8(plaintext[..32].to_owned()).expect("Unable to decode ASCII bytes"));
}

pub fn decrypt_block(prior: &[u8], block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut crypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    ).expect("Unable to initialize crypter");
    crypter.pad(false);

    let mut output = vec![0; key.len() + block.len()];
    let mut count = crypter.update(block, &mut output).expect("Unable to decrypt block");
    count = crypter.finalize(&mut output).expect("Unable to finalize decryption");

    return fixed_xor(output.as_slice(), prior).to_owned();
}

#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    extern crate openssl;

    use rustc_serialize::hex::FromHex;
    use openssl::symm::{Crypter,Cipher,Mode};
    use decrypt_block;

    #[test]
    fn test_decrypt_block() {
        let block = "091230aade3eb330dbaa4358f88d2a6c".from_hex().expect("Unable to decode block");
        let iv = vec![0x00; 16];
        let key = "YELLOW SUBMARINE".as_bytes();
        let expected = "49276d206261636b20616e642049276d".from_hex().expect("Unable to decode expected result");

        let output = decrypt_block(&iv, &block, &key);
        assert_eq!(output[..16].to_owned(), expected);
    }
}
