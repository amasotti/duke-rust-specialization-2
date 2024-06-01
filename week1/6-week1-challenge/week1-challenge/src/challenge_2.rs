use std::collections::HashMap;
use std::fs;
use std::io::Error as IoError;

const MIN_WORD_LENGTH: usize = 5;

pub fn challenge_2() {
    println!("Challenge 2");
    let contents = get_file_content("data/poe_holmes.txt").expect("Could not read file");

    let words = get_words(&contents);
    let cleaned_words = clean_up(words);

    let words_freq = get_words_freq(cleaned_words);

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

fn clean_up(words: Vec<&str>) -> Vec<String> {
    let mut words_cleaned = Vec::new();

    for word in words {
        let mut w = word.to_string();

        // Handle punctuation
        w = w.chars().filter(|c| c.is_alphabetic()).collect();

        // Convert to lowercase
        w = w.to_lowercase();

        // Handle empty strings
        if w.is_empty() { continue; }

        // Handle blacklist and minimum word length
        if is_in_blacklist(&w) { continue; }

        words_cleaned.push(w);
    }

    words_cleaned
}

fn get_words_freq(words: Vec<String>) -> HashMap<String, i32> {
    let mut words_freq = HashMap::new();

    for word in words {
        let count = words_freq.entry(word).or_insert(0);
        *count += 1;
    }

    words_freq
}

fn is_in_blacklist(word: &str) -> bool {
    let blacklist = [
        "the", "and", "of", "to", "a", "in",
        "that", "was", "he", "his", "it", "i",
        "with", "as", "had", "for", "at", "by", "on", "not",
        "be", "this", "but", "which", "from", "or", "have",
        "an", "they", "one", "all", "we", "you", "were", "her",
        "would", "there", "their", "will", "when", "who", "him",
        "been", "more", "out", "up", "into", "if", "no",
        "what", "so", "said", "its", "about", "them",
        "then", "some", "only", "could", "she", "these",
        "two", "may", "first", "any", "other", "do", "like",
        "my", "over", "such"
    ];

    blacklist.contains(&word) || word.len() <= MIN_WORD_LENGTH
}

fn take_n(words_freq: HashMap<String, i32>, n: usize) -> HashMap<String, i32> {
    let mut words_freq_vec: Vec<_> = words_freq.into_iter().collect();
    words_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    words_freq_vec.into_iter().take(n).collect()
}
