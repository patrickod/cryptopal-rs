use std::convert::TryInto;

use rand::Rng;

use oracle::{Oracle, OracleBase, TARGET_SUFFIX_BYTES};

pub struct RandomPrefixTargetSuffixEcbOracle {
    base: OracleBase,
}

impl RandomPrefixTargetSuffixEcbOracle {
    pub fn new() -> Self {
        // generate random-length prefix bytes for each oracle instantiation
        let mut rng = rand::thread_rng();
        let mut prefix = vec![0u8; rng.gen_range(8, 40)];
        rng.fill(prefix.as_mut_slice());

        let base = OracleBase {
            key: "YELLOW SUBMARINE".as_bytes().try_into().expect("bad key"),
            prefix,
            suffix: TARGET_SUFFIX_BYTES.to_vec(),
            use_ecb: true,
        };

        Self { base }
    }

    pub fn verify(&self, guess: &[u8]) -> bool {
        &TARGET_SUFFIX_BYTES[..] == guess
    }
}

impl Oracle for RandomPrefixTargetSuffixEcbOracle {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        self.base.encrypt(&plaintext)
    }
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        self.base.decrypt(&ciphertext)
    }
}
