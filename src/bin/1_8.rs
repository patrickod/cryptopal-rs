extern crate cryptopal;
extern crate hex;

use cryptopal::util::{detect_duplicate_blocks, load_file_as_lines};

fn main() {
    let lines = load_file_as_lines("./data/8.txt");
    let mut lines_with_duplicates = lines.iter().filter(|l| detect_duplicate_blocks(l));
    println!(
        "{}",
        hex::encode(lines_with_duplicates.nth(0).expect("No duplicates found"))
    );
}
