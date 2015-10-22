pub fn repeating_character_xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}
