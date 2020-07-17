extern crate openssl;
extern crate rustc_serialize;
extern crate cryptopal;

use cryptopal::util::load_data_lines;

use std::collections::HashMap;

use rustc_serialize::hex::ToHex;

fn has_duplicate_blocks(line: &[u8]) -> bool {
    let blocks = line.chunks(16 as usize);
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

fn main() {
    let lines = load_data_lines("./data/8.txt");
    let mut lines_with_duplicates = lines.iter().filter(|l| has_duplicate_blocks(l));
    println!("{}", lines_with_duplicates.nth(0).expect("No duplicates found").to_hex());
}
