use std::collections::HashMap;
use rand::seq::SliceRandom;

fn main() {
    // Create a vector of fruits (IMMUTABLE)
    let fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}", fruit_salad);

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    fruit_salad.push("figs");

    // Shuffle the vector
    let mut rng = rand::thread_rng();
    fruit_salad.shuffle(&mut rng);
    println!("Figs added to fruit salad: {:?}", fruit_salad);

    // Remove the last element from the vector
    fruit_salad.pop();
    println!("Last element dropped from fruit salad: {:?}", fruit_salad);

    // Iteration
    for fruit in fruit_salad.iter() {
        println!("Fruit: {}", fruit);
    }

    // Remove an element from the vector
    remove_from_vec("banana", &mut fruit_salad);
    println!("Fruit salad after removing banana: {:?}", fruit_salad);


    // Sort the vector
    sort_vec(&mut fruit_salad);


    // Count the occurrences of each fruit in the vector
    let count = count_fruits_occurrences(&fruit_salad);
    println!("Fruit salad count: {:?}", count);
}


fn sort_vec(vec: &mut Vec<&str>) {
    // Sort alphabetically
    vec.sort();
    println!("Fruit salad sorted: {:?}", vec);
}


fn remove_from_vec(element: &str, vec: &mut Vec<&str>) {
    let index = vec.iter().position(|&x| x == element);
    match index {
        Some(i) => {
            vec.remove(i);
            println!("Element {} removed from vector", element);
        },
        None => println!("Element {} not found in vector", element),
    }
}

fn count_fruits_occurrences(vec: &Vec<&str>) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    for f in vec.iter() {
        let c = count.entry(f.to_string()).or_insert(0);
        *c += 1;
    }
    count
}