use rand::Rng;

use crate::random_key;
use oracle::{Oracle, OracleBase};

fn random_padding_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(5, 11);
    vec![0u8; size]
}
pub struct CbcEcbOracle {
    base: OracleBase,
}

impl CbcEcbOracle {
    pub fn new() -> Self {
        let base = match rand::random::<bool>() {
            true => OracleBase {
                key: random_key(),
                prefix: random_padding_bytes(),
                suffix: random_padding_bytes(),
                use_ecb: true,
                iv: None,
            },
            false => OracleBase {
                key: random_key(),
                prefix: random_padding_bytes(),
                suffix: random_padding_bytes(),
                use_ecb: false,
                iv: Some(random_key()),
            },
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
