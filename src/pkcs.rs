pub fn pad(bytes: &[u8], length: u8 ) -> Vec<u8> {
    let needed_bytes = length - bytes.len() as u8;
    let padding = vec![4u8].iter().cycle().take(needed_bytes);
    let padded = bytes.clone().to_owned().extend_from_slice(&padding);
    return padded;
}

