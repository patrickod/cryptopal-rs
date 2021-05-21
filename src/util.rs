use std;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::slice::Chunks;

use hex;
use oracle::Oracle;
use BLOCK_SIZE;

pub fn english_score(s: &[u8]) -> u32 {
    return s
        .iter()
        .map(|&c| character_score(c))
        .fold(0u32, |sum, c| sum + c as u32);
}

fn character_score(c: u8) -> u32 {
    let c = match std::char::from_u32(c as u32) {
        Some(c) => c,
        None => {
            return 0;
        }
    };
    return match c {
        'z' => 74,
        'q' => 95,
        'x' => 150,
        'j' => 153,
        'k' => 772,
        'v' => 978,
        'b' => 1492,
        'p' => 1929,
        'y' => 1974,
        'g' => 2015,
        'f' => 2228,
        'w' => 2361,
        'm' => 2406,
        'u' => 2758,
        'c' => 2782,
        'l' => 4025,
        'd' => 4253,
        'r' => 5987,
        'h' => 6094,
        's' => 6327,
        'n' => 6749,
        'i' => 6966,
        'o' => 7507,
        'a' => 8167,
        't' => 9056,
        'e' => 12702,
        ' ' => 13000,
        _ => 0,
    };
}

pub fn load_data(path: &str) -> Vec<u8> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut out: Vec<u8> = Vec::new();

    for line in reader.lines() {
        out.extend(line.unwrap().into_bytes());
    }

    out
}

pub fn load_data_lines(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("data not found");
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let decoded = hex::decode(&line.unwrap()).unwrap();
        lines.push(decoded);
    }

    lines
}

// calculate the hamming distance between two equal length slices of u8
pub fn hamming(a: &[u8], b: &[u8]) -> u32 {
    let pairs = a.iter().zip(b.iter());
    return pairs
        .map(|(a, b)| (*a ^ *b).count_ones() as u32)
        .fold(0, |sum, i| sum + i) as u32;
}

pub fn transpose(chunks: &Chunks<u8>, size: u8) -> Vec<Vec<u8>> {
    let mut results: Vec<Vec<u8>> = vec![Vec::new(); size as usize];

    for i in 0..size {
        for c in chunks.to_owned() {
            // The vec definitely exists
            let r = results.get_mut(i as usize).unwrap();

            // however the chunks aren't all guaranteed to be equal length
            match c.get(i as usize) {
                Some(v) => {
                    r.push(v.clone());
                }
                None => (),
            }
        }
    }

    return results;
}

pub fn detect_duplicate_blocks(bytes: &[u8]) -> bool {
    let blocks = bytes.chunks(::BLOCK_SIZE.into());
    let mut frequency: HashMap<&[u8], u8> = HashMap::new();

    for block in blocks {
        let ref mut freq = &mut frequency;
        let count = freq.entry(block).or_insert(0);
        *count += 1;
    }

    for (_, count) in frequency.iter() {
        if *count > 1 {
            return true;
        }
    }
    return false;
}

pub fn calculate_oracle_block_size<T: Oracle>(oracle: &T) -> Option<usize> {
    let initial_length = oracle.encrypt(&vec![0u8; 1]).len();
    for n in 1..32 {
        let length = oracle.encrypt(&vec![0; n]).len();
        if length != initial_length {
            return Some(length - initial_length);
        }
    }
    None
}

/// determine the length of the oracle's prefix||suffix by encrypting
/// iteratively larger plaintexts until we observe a BLOCK_SIZE change in
/// ciphertext size.
/// at this point we know: len(plaintext) + len(prefix||oracle) % BLOCK_SIZE == 0
/// pkcs7 padding adds a complete BLOCK_SIZE of padding in this case also which
/// we subtract to ascertain the prefix||suffix size
pub fn calculate_total_payload_length<T: Oracle>(oracle: &T) -> Option<usize> {
    let initial_length = oracle.encrypt(&[]).len();

    for n in 1..32 {
        let length = oracle.encrypt(&vec![0; n]).len();
        if length != initial_length {
            return Some(initial_length - n - BLOCK_SIZE as usize);
        }
    }
    None
}

// determine the number of whole prefix blocks by encrypting two discrete
// ciphertexts and counting the nuber of duplicate blocks
pub fn calculate_prefix_block_count<T: Oracle>(oracle: &T) -> Option<usize> {
    oracle
        .encrypt(&[0])
        .chunks(BLOCK_SIZE.into())
        .zip(oracle.encrypt(&[1]).chunks(BLOCK_SIZE.into()))
        .position(|(a, b)| a != b)
}

// determine the total number of bytes prepended to our plaintext in a given
// oracle
pub fn calculate_prefix_length<T: Oracle>(oracle: &T) -> Option<usize> {
    let prefix_block_count: usize = calculate_prefix_block_count(oracle).unwrap();
    let prefix_offset: usize = prefix_block_count * BLOCK_SIZE;

    let f = |c: u8| -> Option<usize> {
        let v = vec![c; BLOCK_SIZE];
        let range = prefix_offset..(prefix_offset + BLOCK_SIZE);
        let initial = &oracle.encrypt(&v)[range];
        for n in 0..BLOCK_SIZE {
            let c = &oracle.encrypt(&v[n + 1..]);
            if initial != &c[prefix_offset..(prefix_offset + BLOCK_SIZE)] {
                return Some(n);
            }
        }
        None
    };

    Some(prefix_offset + cmp::min(f(b'a').unwrap(), f(b'b').unwrap()))
}

#[cfg(test)]
mod tests {
    use oracle::*;
    use util::*;
    use BLOCK_SIZE;

    #[test]
    fn test_hamming() {
        let a = "this is a test".as_bytes();
        let b = "wokka wokka!!!".as_bytes();

        assert_eq!(hamming(&a, &b), 37);
    }

    #[test]
    fn test_transpose() {
        let original = vec![1, 2, 3, 4, 5];
        let chunks = original.chunks(2);
        let transposed = transpose(&chunks, 2);

        let mut iter = transposed.iter();

        assert_eq!(iter.next().unwrap(), &[1, 3, 5]);
        assert_eq!(iter.next().unwrap(), &[2, 4]);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_detect_duplicate_blocks() {
        assert_eq!(
            false,
            detect_duplicate_blocks(&[vec![0; 16], vec![1; 16]].concat())
        );
        assert_eq!(
            true,
            detect_duplicate_blocks(&[vec![0; 16], vec![0; 16]].concat())
        );
        assert_eq!(
            false,
            detect_duplicate_blocks(&[vec![0; 16], vec![1; 16], vec![2; 16]].concat())
        );
    }

    #[test]
    fn test_oracle_detect_ecb() {
        for _ in 0..100 {
            let o = CbcEcbOracle::new();
            // need to create at least 2 blocks worth of content in addition to
            // the padding being added by oracle to observe duplicates
            let input = vec![b'a'; 50];
            let guess = detect_duplicate_blocks(&o.encrypt(&input));
            assert_eq!(true, o.verify(guess));
        }
    }

    #[test]
    fn test_calculate_oracle_block_size() {
        let oracle = UnknownSuffixEcbOracle::new();
        assert_eq!(
            calculate_oracle_block_size(&oracle).unwrap(),
            BLOCK_SIZE.into()
        );
    }

    #[test]
    fn test_calculate_unknown_suffix_oracle_payload_length() {
        let oracle = UnknownSuffixEcbOracle::new();
        assert_eq!(
            calculate_total_payload_length(&oracle).unwrap(),
            TARGET_SUFFIX_BYTES.len()
        );
    }

    #[test]
    fn test_calculate_prefix_block_count() {
        let oracle = UnknownSuffixEcbOracle::new();
        assert_eq!(0, calculate_prefix_block_count(&oracle).unwrap());
    }

    #[test]
    fn test_calculate_profile_oracle_payload_length() {
        let oracle = ProfileOracle::new();
        assert_eq!(
            calculate_total_payload_length(&oracle).unwrap(),
            "email=&uid=10&role=user".len()
        )
    }

    #[test]
    fn test_calculate_profile_oracle_prefix_block_count() {
        let oracle = ProfileOracle::new();
        assert_eq!(0, calculate_prefix_block_count(&oracle).unwrap());
    }

    #[test]
    fn test_calculate_profile_oracle_prefix_length() {
        let oracle = ProfileOracle::new();
        assert_eq!("email=".len(), calculate_prefix_length(&oracle).unwrap());
    }

    #[test]
    fn test_calculate_random_prefix_unknown_suffix_oracle_payload_length() {
        let o = RandomPrefixTargetSuffixEcbOracle::new();
        let payload_length = calculate_total_payload_length(&o).unwrap();
        let prefix_length = calculate_prefix_length(&o).unwrap();
        assert_eq!(TARGET_SUFFIX_BYTES.len(), (payload_length - prefix_length));
    }
}
