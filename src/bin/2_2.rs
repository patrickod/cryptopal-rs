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
use cryptopal::xor::fixed_xor_mut;

fn main() {
    let base64 = load_data_single_line("data/10.txt").expect("Unable to load 10.txt");
    let data = base64.from_base64().expect("Unable to b64decode data");
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = vec![0x00; 16];

    let mut block_output = vec![0u8; 32];
    let mut plaintext: Vec<u8> = vec![];

    let mut chunks = data.chunks(16 as usize);
    let first = chunks.next().expect("Unable to read first 16 bytes from ciphertext");

    println!("IV: {:?}
Key: {:?}
First: {:?}", iv.to_hex(), key.to_hex(), first.to_hex());

    decrypt_block(&iv, first, &key, block_output.as_mut_slice());


    println!("{}", block_output[1..16].to_owned().to_hex());

    // for chunk in chunks {
    //     let _ = encrypt_block(iv.as_slice(), chunk, &key, block_output.as_mut_slice());
    //     plaintext.extend(block_output[1..16].to_owned());
    //     println!("output: {:?}", plaintext);
    // }
}

fn decrypt_block(prior: &[u8], block: &[u8], key: &[u8], output: &mut [u8]) {
    let mut crypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    ).expect("Unable to initialize crypter");
    crypter.pad(false);

    let mut count = crypter.update(block, output).expect("Unable to decrypt block");
    count = crypter.finalize(output).expect("Unable to finalize decryption");

    println!("Pre-xor: {}", output[1..16].to_owned().to_hex());

    fixed_xor_mut(output, prior);
}

#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    extern crate openssl;

    use rustc_serialize::hex::FromHex;
    use openssl::symm::{Crypter,Cipher,Mode};

    #[test]
    fn test_decrypt_block() {
        let block = "091230aade3eb330dbaa4358f88d2a6c".from_hex().expect("Unable to decode block");
        let iv = "0000000000000000".as_bytes();
        let key = "YELLOW SUBMARINE".as_bytes();
        let mut output = vec![0u8; 32];
        let expected = "49276d206261636b20616e642049276d".from_hex().expect("Unable to decode expected result");

        let mut crypter = Crypter::new(
            Cipher::aes_128_ecb(),
            Mode::Decrypt,
            key,
            Some(iv)
        ).expect("Unable to initialize crypter");
        crypter.pad(false);

        crypter.update(block.as_slice(), output.as_mut_slice()).expect("Unable to decrypt block");
        crypter.finalize(output.as_mut_slice()).expect("Unable to finalize");

        // TODO there's gotta be a better way to take the first 16 bytes
        let mut output_chunks = output.chunks(16 as usize);
        let first = output_chunks.next().expect("Unable to read first 16 bytes from ciphertext");
        assert_eq!(first.to_owned(), expected);
    }
}
