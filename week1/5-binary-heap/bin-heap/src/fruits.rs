use std::cmp::{Ord, Ordering};
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub enum Fruit {
	Coconut, // Which I love and will get higher priority
	Other(String),
}

/// Implementing Ord for Fruit
///
/// This is done by comparing the two fruits and returning the ordering
/// In this fictional example, I am giving higher priority to Coconut
/// and lower priority to Other
impl Ord for Fruit {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Fruit::Coconut, Fruit::Coconut) => Ordering::Equal,
			(Fruit::Coconut, Fruit::Other(_)) => Ordering::Greater,
			(Fruit::Other(_), Fruit::Coconut) => Ordering::Less,
			_ => Ordering::Equal,
		}
	}
}

/// Implementing PartialOrd for Fruit
///
/// This is done by delegating the implementation to the Ord trait
impl PartialOrd for Fruit {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Display for Fruit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Fruit::Coconut => write!(f, "Coconuts 🥥"),
			Fruit::Other(name) => write!(f, "{}", name),
		}
	}
}

pub fn fruit_vec() -> Vec<Fruit> {
	vec![
		Fruit::Coconut,
		Fruit::Other("Apple".to_string()),
		Fruit::Other("Banana".to_string()),
		Fruit::Other("Cherry".to_string()),
		Fruit::Other("Date".to_string()),
		Fruit::Other("Elderberry".to_string()),
		Fruit::Other("Fig".to_string()),
		Fruit::Other("Grape".to_string()),
		Fruit::Other("Honeydew".to_string()),
		Fruit::Other("Icaco".to_string()),
		Fruit::Other("Jackfruit".to_string()),
		Fruit::Other("Kiwi".to_string()),
		Fruit::Other("Lemon".to_string()),
	]
}
