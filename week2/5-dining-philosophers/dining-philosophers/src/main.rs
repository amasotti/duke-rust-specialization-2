/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources (forks in the classical problem), risking deadlock.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

mod philosopher;
mod utils;

use std::sync::{Arc};
use std::thread;
use std::time::Instant;
use philosopher::{Philosopher};

use crate::utils::init_forks;

fn init_philosophers() -> Vec<(&'static str, usize, usize)>{
    vec![
        ("Hans Gadamer", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Thomas Piketty", 3, 0),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Seneca", 1, 2),
        ("Thales of Miletus", 2, 3),
        ("Zeno of Elea", 3, 0),
        ("Parmenides", 0, 1),
        ("Anaxagoras", 1, 2),
        ("Protagoras", 2, 3),
        ("Hippocrates", 3, 0),
        ("Hipparchus", 0, 1),
        ("Ptolemy", 1, 2),
        ("Archimedes", 2, 3),
        ("Euclid", 3, 0),
        ("Aristarchus", 0, 1),
        ("Eratosthenes", 1, 2),
        ("Hipparchus", 2, 3),
        ("Ptolemy", 3, 0),
        ("Heraclitus", 0, 1),
        ("Democritus", 1, 2),
        ("Diogenes", 2, 3),
        ("Epicuro", 3, 0),
    ]
}


fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    //we only have 4 forks at the table
    let forks = init_forks();

    let philosophers = init_philosophers()
        .into_iter()
        .enumerate()
        .map(|(id, (name, left, right))| {
            Philosopher::new(
                id as u32,
                name,
                Arc::clone(&forks[left]),
                Arc::clone(&forks[right]),
            )
        })
        .collect::<Vec<_>>();

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}

