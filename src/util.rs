use std;

pub fn english_score(s: &[u8]) -> i32 {
    return s.iter().map (|&c| character_score(c)).fold(0i32, |sum, c| sum + c as i32);
}

fn character_score(c: u8) -> i32 {
    let character = match std::char::from_u32(c as u32) {
        Some(character) => character,
        None => { return 0; }
    };

    let mut score: i32 = 0;

    if c > 37 && c < 127 {
        score = score + 1
    }

    return match character {
        ' ' => score + 5,
        'e' => score + 5,
        't' => score + 5,
        'a' => score + 4,
        'o' => score + 4,
        'i' => score + 4,

        _ => score
    };
}


// calculate the hamming distance between two equal length slices of u8
fn hamming(a: &[u8], b: &[u8]) -> u32 {
    let pairs = a.iter().zip(b.iter());
    return pairs.map ( |(a,b)|
        (*a ^ *b).count_ones() as u32
    ).fold(0, ( |sum, i| sum + i )) as u32;
}

#[test]
fn test_hamming() {
    let a = "this is a test".as_bytes();
    let b = "wokka wokka!!!".as_bytes();

    assert_eq!(hamming(&a, &b), 37);
}
