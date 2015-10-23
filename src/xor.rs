pub fn repeating_character_xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}

pub fn repeating_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let cycle = b.iter();

    return cycle.zip(a.iter()).map ( |(a, b)| *a ^ *b ).collect();
}
