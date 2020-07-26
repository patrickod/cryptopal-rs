pub fn repeating_character_xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}

pub fn repeating_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let cycle = b.iter().cycle();

    return cycle.zip(a.iter()).map(|(a, b)| *a ^ *b).collect();
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    return a.iter().zip(b.iter()).map(|(x, y)| *x ^ *y).collect();
}

#[cfg(test)]
mod tests {
    use hex;
    use xor::*;

    #[test]
    fn test_repeating_xor() {
        let a = vec![0u8, 0u8];
        let b = vec![1u8];

        let r = repeating_xor(&a, &b);
        assert_eq!(r, &[1u8, 1u8]);
    }

    #[test]
    fn test_xor() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let c = hex::decode("746865206b696420646f6e277420706c6179").unwrap();

        let r = xor(&a, &b);
        assert_eq!(c, r);
    }
}
