use std::str;

use oracle::{Oracle, OracleBase};
use random_key;

pub struct CbcTargetOracle {
    base: OracleBase,
}

impl CbcTargetOracle {
    pub fn new() -> Self {
        let base = OracleBase {
            key: random_key(),
            prefix: "comment1=cooking%20MCs;userdata=".as_bytes().to_vec(),
            suffix: ";comment2=%20like%20a%20pound%20of%20bacon"
                .as_bytes()
                .to_vec(),
            use_ecb: false,
        };

        Self { base }
    }

    pub fn verify(&self, guess: &[u8]) -> bool {
        match str::from_utf8(guess) {
            Ok(s) => s.contains(";admin=true;"),
            Err(_) => false,
        }
    }
}

impl Oracle for CbcTargetOracle {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let sanitized = str::from_utf8(plaintext)
            .unwrap()
            .replace(";", "_")
            .replace("=", "_");
        self.base.encrypt(&sanitized.as_bytes())
    }
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        self.base.decrypt(&ciphertext)
    }
}
