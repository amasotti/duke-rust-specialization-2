use std::collections::HashMap;
use std::ops::Range;

use rand::prelude::SliceRandom;
use rand::Rng;

fn main() {
    let cipher = homophonic_cipher();
    println!("{:?}", cipher);

    //  Test
    let plain_text = String::from("Mi chiamo Toni e Rust è una figata");
    let encrypted = encrypt(&cipher, &plain_text);

    println!("Plain text: {}", plain_text);
    println!("Encrypted text: {}", encrypted);

    // Test 2
    let plain_text = String::from("Aaaaa bbbb cccc dddd eeee ffff gggg");
    let encrypted = encrypt(&cipher, &plain_text);

    println!("Plain text: {}", plain_text);
    println!("Encrypted text: {}", encrypted);


}

fn encrypt(cipher: &HashMap<char, Vec<char>>, plain_text: &String) -> String {
    let mut encrypted = String::new();
    let mut rng = rand::thread_rng();
    for c in plain_text.chars() {
        if c.is_alphabetic() {
            let random_chars = cipher.get(&c).unwrap();
            let random_char = random_chars[rng.gen_range(0..random_chars.len())];
            encrypted.push(random_char);
        } else {
            encrypted.push(c);
        }
    }
    encrypted
}

fn homophonic_cipher() -> HashMap<char, Vec<char>> {
    let mut cipher = HashMap::new();
    let alphabet: Vec<char> = "abcdeèfghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut rng = rand::thread_rng();
    let mut shuffled_alphabet = alphabet.clone();
    shuffled_alphabet.shuffle(&mut rng);

    for &c in &alphabet {
        let mut random_chars = Vec::new();
        for _ in 0..2 {
            random_chars.push(shuffled_alphabet[rng.gen_range(0..shuffled_alphabet.len())]);
        }
        cipher.insert(c, random_chars);
    }

    cipher
}
