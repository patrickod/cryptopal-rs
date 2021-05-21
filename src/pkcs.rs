use std::iter::*;

pub fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let remainder = bytes.len() % block_size;
    let padding_value = block_size - remainder;
    let padding: Vec<u8> = repeat(padding_value as u8).take(padding_value).collect();

    [bytes.to_owned(), padding].concat()
}

pub fn strip(bytes: &[u8]) -> Result<&[u8], &'static str> {
    let padding_value = match bytes.last() {
        Some(&b) => b,
        None => return Err("empty"),
    };
    if padding_value as usize > bytes.len() {
        return Err("invalid padding");
    }
    let content_length = bytes.len() - padding_value as usize;
    match bytes[content_length..].iter().all(|&c| c == padding_value) {
        true => Ok(&bytes[..content_length]),
        false => Err("invalid padding"),
    }
}

#[cfg(test)]
mod tests {
    use pkcs::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pad() {
        assert_eq!(pad("TEST".as_bytes(), 8), [84, 69, 83, 84, 4, 4, 4, 4]);

        assert_eq!(pad("TESTS".as_bytes(), 8), [84, 69, 83, 84, 83, 3, 3, 3]);

        assert_eq!(
            pad("ICE ICE BABY".as_bytes(), 12),
            [
                73, 67, 69, 32, 73, 67, 69, 32, 66, 65, 66, 89, 12, 12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12
            ]
        )
    }

    #[test]
    fn test_strip() {
        // round-trip through our own padding function
        let plaintext = "ICE ICE BABY".as_bytes();
        let correctly_padded = pad(plaintext, 8);
        assert_eq!(Ok(plaintext), strip(&correctly_padded));

        // incorrect padding value
        let p2 = [plaintext, &repeat(5u8).take(4).collect::<Vec<u8>>()].concat();
        assert_eq!(Err("invalid padding"), strip(&p2));
    }
}
