use oracle::{Oracle, OracleBase, TARGET_SUFFIX_BYTES};
use std::convert::TryInto;

pub struct UnknownSuffixEcbOracle {
    base: OracleBase,
}

impl UnknownSuffixEcbOracle {
    pub fn new() -> Self {
        let base = OracleBase {
            key: "YELLOW SUBMARINE".as_bytes().try_into().expect("bad key"),
            prefix: vec![],
            suffix: TARGET_SUFFIX_BYTES.to_vec(),
            use_ecb: true,
            iv: None,
        };

        Self { base }
    }

    pub fn verify(&self, guess: &[u8]) -> bool {
        &TARGET_SUFFIX_BYTES[..] == guess
    }
}

impl Oracle for UnknownSuffixEcbOracle {
    fn encrypt(&self, p: &[u8]) -> Vec<u8> {
        self.base.encrypt(&p)
    }
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        self.base.decrypt(&ciphertext)
    }
}
