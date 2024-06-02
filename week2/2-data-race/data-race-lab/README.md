# 2.2 Data Race Demo

## Instructions

- Copy the given code snippet into your src/main.rs file.
- Uncomment the first block of code that uses Mutex to protect the data vector.
- Comment out the second block of code that tries to capture a mutable reference in multiple threads.

## Challenges

- Can you modify the code to use read-write locks (RwLock) instead of a Mutex? What are the advantages and disadvantages?
- How would you handle potential deadlocks in a more complex application that uses multiple Mutexes?
- Can you extend the code to use conditional variables to synchronize thread execution?