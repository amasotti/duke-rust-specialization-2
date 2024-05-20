use std::collections::BinaryHeap;
use rand::seq::SliceRandom;
use crate::fruits::Fruit;

mod fruits;

fn generate_random_fruit_salad(fruits: Vec<Fruit>) -> BinaryHeap<Fruit> {
	let mut rng = rand::thread_rng();
	let mut salad: BinaryHeap<Fruit> = BinaryHeap::new();


	let mut coconut_count = 0;
	while coconut_count < 2 {
		let fruit = fruits.choose(&mut rng).unwrap();
		if *fruit == Fruit::Coconut {
			coconut_count += 1;
			salad.push(Fruit::Coconut);
		} else {
			salad.push(Fruit::Other(fruit.to_string()));
		}
	}

	salad
}


fn main() {
    let fruits = fruits::fruit_vec();
	let salad = generate_random_fruit_salad(fruits);
	println!("Random fruit salad with at least 2 coconuts:");

	for fruit in salad.into_sorted_vec() {
		println!("{}", fruit);
	}
}
