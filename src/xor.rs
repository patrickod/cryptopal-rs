pub fn repeating_character_xor(a: &[u8], b: u8) -> Vec<u8> {
    return a.iter().map(|x| *x ^ b).collect();
}

pub fn repeating_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let cycle = b.iter().cycle();

    return cycle.zip(a.iter()).map ( |(a, b)| *a ^ *b ).collect();
}

#[test]
fn test_repeating_xor() {
    let a = vec![0u8, 0u8];
    let b = vec![1u8];

    let r = repeating_xor(&a, &b);
    assert_eq!(r, &[1u8, 1u8]);
}
