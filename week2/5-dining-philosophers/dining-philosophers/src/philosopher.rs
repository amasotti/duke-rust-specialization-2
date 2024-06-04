use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A fork is a shared resource that can be picked up by a philosopher.
///
/// It is protected by a mutex to ensure that
/// only one philosopher can pick it up at a time.
pub struct Fork {
    pub id: u32,
    pub mutex: Mutex<()>,
}

/// A philosopher is a thread that can pick up two forks and eat.
pub struct Philosopher {
    pub id: u32,
    pub name: String,
    pub left_fork: Arc<Fork>,
    pub right_fork: Arc<Fork>,
}

impl Philosopher {
    pub fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    pub fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = first_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _second_guard = second_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        // Simulate release of forks, since this function terminates here
        // and the mutexes will be dropped.
        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}
