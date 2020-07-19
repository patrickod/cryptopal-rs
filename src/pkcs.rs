use std::iter::*;

pub fn pad(bytes: &[u8], block_size: u8 ) -> Vec<u8> {
    let remainder = bytes.len() % block_size as usize;
    let p_value = block_size - remainder as u8;
    let padding: Vec<u8> = repeat(p_value).take(p_value.into()).collect();

    [
        bytes.to_owned(),
        padding
    ].concat()
}

#[test]
fn test_pad() {
    assert_eq!(
        pad("TEST".as_bytes(), 8),
        [84, 69, 83, 84, 4, 4, 4, 4]
    );

    assert_eq!(
        pad("TESTS".as_bytes(), 8),
        [84, 69, 83, 84, 83, 3, 3, 3]
    );

    assert_eq!(
        pad("ICE ICE BABY".as_bytes(), 12),
        [73, 67, 69, 32, 73, 67, 69, 32, 66, 65, 66, 89,
         12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12]
    )
}
