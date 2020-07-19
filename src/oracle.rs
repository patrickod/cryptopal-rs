use std::convert::TryInto;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc, Ecb};
use rand::prelude::*;

use {pkcs, AesKey, BLOCK_SIZE};
type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

const UNKNOWN_SUFFIX_BYTES: &'static [u8] = &base64!(
    "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
    YnkK"
);

fn random_key() -> AesKey {
    let mut key: AesKey = [0u8; 16];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key.clone()
}

fn padding_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(5, 11);
    vec![0u8; size]
}

pub trait Oracle {
    fn encrypt(&self, p: &[u8]) -> Vec<u8>;
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
            BLOCK_SIZE.into(),
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
}

impl OracleBase {
    fn check_ecb(&self, guess: bool) -> bool {
        self.use_ecb == guess
    }
}

pub struct CbcEcbOracle {
    base: OracleBase,
}

impl CbcEcbOracle {
    pub fn new() -> Self {
        let base = OracleBase {
            key: random_key(),
            prefix: padding_bytes(),
            suffix: padding_bytes(),
            use_ecb: rand::random::<bool>(),
        };

        Self { base }
    }

    pub fn encrypt(&self, p: &[u8]) -> Vec<u8> {
        self.base.encrypt(&p)
    }

    pub fn verify(&self, guess: bool) -> bool {
        self.base.check_ecb(guess)
    }
}

pub struct UnknownSuffixEcbOracle {
    base: OracleBase,
}

impl UnknownSuffixEcbOracle {
    pub fn new() -> Self {
        let base = OracleBase {
            key: "YELLOW SUBMARINE".as_bytes().try_into().expect("bad key"),
            prefix: vec![],
            suffix: UNKNOWN_SUFFIX_BYTES.to_vec(),
            use_ecb: true,
        };

        Self { base }
    }

    pub fn verify(&self, guess: &[u8]) -> bool {
        &UNKNOWN_SUFFIX_BYTES[..] == guess
    }
}

impl Oracle for UnknownSuffixEcbOracle {
    fn encrypt(&self, p: &[u8]) -> Vec<u8> {
        self.base.encrypt(&p)
    }
}

#[cfg(test)]
mod test {
    use oracle::CbcEcbOracle;
    use util::has_duplicate_blocks;

    #[test]
    fn test_oracle_detect_ecb() {
        for _ in 0..100 {
            let o = CbcEcbOracle::new();
            // need to create at least 2 blocks worth of content in addition to
            // the padding being added by oracle to observe duplicates
            let input = vec![b'a'; 50];
            let guess = has_duplicate_blocks(&o.encrypt(&input));
            assert_eq!(true, o.verify(guess));
        }
    }
}
