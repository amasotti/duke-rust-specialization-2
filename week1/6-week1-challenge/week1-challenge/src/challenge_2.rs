use std::collections::HashMap;
use std::fs;
use std::io::Error as IoError;

pub fn challenge_2() {
    println!("Challenge 2");
    let contents = get_file_content("data/poe_holmes.txt").expect("Could not read file");

    let words = get_words(&contents);

    let words_freq = get_words_freq(words);

    let top_10_words = take_n(words_freq, 10);

    // Print first 10 words
    for (word, freq) in top_10_words {
        println!("{}: {}", word, freq);
    }
}

fn get_file_content(path: &str) -> Result<String, IoError> {
    fs::read_to_string(path)
}

fn get_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

fn get_words_freq(words: Vec<&str>) -> HashMap<String, i32> {
    let mut words_freq = HashMap::new();

    for word in words {
        let count = words_freq.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    words_freq
}

fn take_n(words_freq: HashMap<String, i32>, n: usize) -> HashMap<String, i32> {
    let mut words_freq_vec: Vec<_> = words_freq.into_iter().collect();
    words_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    words_freq_vec.into_iter().take(n).collect()
}
