mod playlist;

/// Example of using a Vec
fn vec_example() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    println!("v = {:?}", v);
}

/// Example of using a vecDeque
///
/// A VecDeque is a double-ended queue implemented with a growable ring buffer.
/// The processing order is usually FIFO (First In First Out)
fn vec_deque_example() {
    use std::collections::VecDeque;

    let mut tasks = VecDeque::new();

    tasks.push_back("task1");
    tasks.push_back("task2");

    println!("tasks = {:?}", tasks);

    tasks.push_front("task0");
    tasks.push_front("task-1");

    println!("tasks = {:?}", tasks);

    while let Some(task) = tasks.pop_front() {
        println!("Processing task: {}", task);
        println!("\tQueue Length: {}", tasks.len());
        println!("\tQueue capacity: {}", tasks.capacity());
    }
}

fn main() {
    vec_example();
    vec_deque_example();
    playlist::linked_list_example();
}
