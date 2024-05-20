/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn pretty_print(freq: &Vec<(String, i32)>) {
	println!("| Word      | Frequency |");
	println!("|-----------|-----------|");
	for (word, frequency) in freq {
		println!("| {:<9} | {:<9} |", word, frequency);
	}
	println!("|-----------|-----------|");

}

#[allow(dead_code)]
fn get_numbers_from_user() -> Vec<i32> {
	let mut numbers = Vec::new();

	loop {
		println!("Enter a number (0 to stop): ");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		let number: i32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid number. Please try again.");
				continue;
			}
		};

		if number == 0 {
			break;
		}

		numbers.push(number);
	}

	numbers
}

fn get_user_input() -> String {
	let mut input = String::new();
	println!("Enter a sentence: ");
	std::io::stdin().read_line(&mut input).expect("Failed to read line");
	input
}

fn get_text_from_file(file_name: &str) -> String {
	use std::fs::File;
	use std::io::Read;

	let mut file = File::open(file_name).expect("File not found");
	let mut text = String::new();
	file.read_to_string(&mut text).expect("Failed to read file");

	text.to_lowercase()
}


fn get_w_frequencies(sentence: &str) ->HashMap<String, i32> {
	let mut frequencies = HashMap::new();
	let words = sentence.split_whitespace();

	for word in words {
		let frequency = frequencies.entry(word.to_string()).or_insert(0);
		*frequency += 1;
	}

	frequencies
}

/// We cannot sort the HashMap directly, so we convert it to a Vec and sort it.
///
/// the primary function of a HashMap is quick safe access, and it's based
/// on the hash keys distributed across buckets, which are not suitable for sorting.
fn sort_word_frequencies(frequencies: HashMap<String, i32>, max: usize) -> Vec<(String, i32)> {
	let mut sorted_freq: Vec<(String, i32)> = frequencies.into_iter().collect();
	sorted_freq.sort_by(|a, b| b.1.cmp(&a.1));

	if max < sorted_freq.len() {
		sorted_freq.truncate(max);
	}

	sorted_freq
}

fn freq_to_file(sorted_freq: &Vec<(String, i32)>) {
	let mut output = String::new();
	output.push_str("Word,Frequency\n");
	for (word, frequency) in sorted_freq {
		let cleaned_word = word
			.replace(",", "")
			.replace("\n", "")
			.replace("\r", "")
			.replace("\t", "")
			.trim()
			.to_string();
		output.push_str(&format!("{},{}\n", cleaned_word, frequency));
	}

	std::fs::write("./data/frequencies.csv", output).expect("Unable to write file");
}


fn main() {
	//let text = get_user_input();

	// A bit more: Read a text file and count the frequency of each word.
	let text = get_text_from_file("./data/poe_holmes.txt");

	let frequencies = get_w_frequencies(&text);

	// Sort the frequencies and print the top 15 (useful if the text is long)
	let sorted_freq = sort_word_frequencies(frequencies, 150);


	println!("Your input: {}", text);
	pretty_print(&sorted_freq);

	// Save the sorted frequencies to a file

	freq_to_file(&sorted_freq);
}


