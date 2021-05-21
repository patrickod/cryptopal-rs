use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc, Ecb};

use crate::{pkcs, random_key, AesKey, BLOCK_SIZE};

// 2-11 ECB/CBC oracle
mod cbc_ecb_oracle;
// 2-12 ECB byte-at-a-time (simple)
mod unknown_suffix_ecb_oracle;
// 2-13 ECB cut-and-paste
pub mod profile;
mod profile_oracle;
// 2-14 ECB byte-at-a-time (harder)
mod random_prefix_target_suffix_ecb_oracle;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

// The target suffix bytes for challenges 2-12 & 2-14
pub const TARGET_SUFFIX_BYTES: &'static [u8] = &base64!(
    "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
    YnkK"
);
pub trait Oracle {
    fn encrypt(&self, p: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

pub struct OracleBase {
    key: AesKey,
    prefix: Vec<u8>,
    suffix: Vec<u8>,
    use_ecb: bool,
}

impl Oracle for OracleBase {
    fn encrypt(&self, p: &[u8]) -> Vec<u8> {
        let key = &self.key;
        let plaintext = pkcs::pad(
            &[self.prefix.to_owned(), p.to_owned(), self.suffix.to_owned()].concat(),
            BLOCK_SIZE,
        );

        if self.use_ecb {
            return Aes128Ecb::new_var(key, Default::default())
                .expect("ECB panic")
                .encrypt_vec(&plaintext);
        }
        Aes128Cbc::new_var(key, &random_key())
            .expect("CBC panic")
            .encrypt_vec(&plaintext)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        if self.use_ecb {
            let decrypter =
                Aes128Ecb::new_var(&self.key, Default::default()).expect("ECB decrypt panic");
            decrypter.decrypt_vec(&ciphertext).expect("bad ecb decrypt")
        } else {
            let decrypter =
                Aes128Cbc::new_var(&self.key, &random_key()).expect("ECB decrypt panic");
            decrypter.decrypt_vec(&ciphertext).expect("bad cbc decrypt")
        }
    }
}

impl OracleBase {
    pub fn check_ecb(&self, guess: bool) -> bool {
        self.use_ecb == guess
    }
}

// convenience exports
pub use self::cbc_ecb_oracle::CbcEcbOracle;
pub use self::unknown_suffix_ecb_oracle::UnknownSuffixEcbOracle;
pub use self::profile_oracle::ProfileOracle;
pub use self::random_prefix_target_suffix_ecb_oracle::RandomPrefixTargetSuffixEcbOracle;