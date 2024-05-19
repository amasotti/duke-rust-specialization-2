//! # Fruit Salad
//!
//! `fruit-cli` is a collection of utilities to create and manage fruit salads.

use rand::seq::SliceRandom;

/// Main lib function for the fruit salad cli
/// Given the wished number of fruits, it will create a salad with random fruits
pub fn create_salad(n_fruits: usize) -> Vec<String> {
    let mut fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "pear",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "watermelon",
    ];

    let mut salad: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    fruits.shuffle(&mut rng);

    fruits
        .into_iter()
        .take(n_fruits)
        .for_each(|fruit| salad.push(fruit.to_string()));

    salad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let salad = create_salad(3);
        assert_eq!(salad.len(), 3);
    }
}
