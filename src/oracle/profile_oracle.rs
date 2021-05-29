use std::convert::TryInto;
use std::str;

use oracle::profile::Profile;
use oracle::{Oracle, OracleBase};

pub struct ProfileOracle {
    base: OracleBase,
}

impl ProfileOracle {
    pub fn new() -> Self {
        let base = OracleBase {
            key: "YELLOW SUBMARINE".as_bytes().try_into().expect("bad key"),
            prefix: vec![],
            suffix: vec![],
            use_ecb: true,
            iv: None,
        };
        Self { base }
    }

    pub fn verify(&self, guess: &[u8]) -> bool {
        let s = match str::from_utf8(guess) {
            Ok(s) => s,
            Err(_) => return false,
        };
        match Profile::parse(s) {
            Ok(p) => p.role.eq("admin"),
            Err(_) => false,
        }
    }
}

impl Oracle for ProfileOracle {
    fn encrypt(&self, p: &[u8]) -> Vec<u8> {
        let p = Profile::for_email(str::from_utf8(p).unwrap());
        self.base.encrypt(p.serialize().as_bytes())
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        self.base.decrypt(ciphertext)
    }
}
