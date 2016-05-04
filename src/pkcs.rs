use std::iter::*;

pub fn pad(bytes: &[u8], length: u8 ) -> Vec<u8> {
    let needed_bytes = length - bytes.len() as u8;
    let padding: Vec<u8> = repeat(4u8).take(needed_bytes as usize).collect();
    let mut result = bytes.clone().to_owned();
    result.extend_from_slice(&padding);
    return result;
}

#[test]
fn test_pad() {
    let input = "TEST".as_bytes();
    let output = pad(input, 8);
    assert_eq!(output, [84, 69, 83, 84, 4, 4, 4, 4]);
}
