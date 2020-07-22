extern crate hex;


pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    return a.iter().zip(b.iter()).map(|(x,y)| *x ^ *y).collect();
}

pub fn main() {
    let x1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let x2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    println!("{}", hex::encode(&xor(&x1, &x2)));
}
