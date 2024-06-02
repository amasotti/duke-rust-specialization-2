// Mutex that protects the data vector, and then we spawn three threads
//that each acquire a lock on the mutex and modify an element of the vector.
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

/// This function demonstrates the usage of `Mutex` and `Arc` in a multi-threaded environment.
/// It creates a vector protected by a `Mutex`, then spawns multiple threads to modify the vector
/// concurrently. The `Arc` (Atomic Reference Counted) is used to share ownership of the `Mutex`
/// across multiple threads safely. Each thread locks the `Mutex`, modifies the vector, and then
/// releases the lock. The main thread waits for all spawned threads to complete before printing
/// the modified vector.
fn main() {
    default();
    rwlock_replacement();
    deadlock_handling();
    conditional_synchronization();
}

fn default() {
    // Use Arc to share ownership of the Mutex across threads
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Create a vector to hold the handles of the spawned threads
    let handles: Vec<_> = (0..3).map(|i| {
        // Clone the Arc to increase the reference count
        let data = Arc::clone(&data);
        // Spawn a new thread
        thread::spawn(move || {
            // Lock the Mutex to get access to the data
            let mut data = data.lock().unwrap();
            // Modify the data at index i
            data[i] += 1;
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the modified vector
    println!("{:?}", data.lock().unwrap());
}


/// Challenge 1 - Use RwLock instead of Mutex
fn rwlock_replacement() {
    // Use Arc to share ownership of the RwLock across threads
    // RwLock allows multiple threads to read the data concurrently, it's used
    // for read-heavy workloads where the data is read more often than it's written.
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Create a vector to hold the handles of the spawned threads
    let handles: Vec<_> = (0..3).map(|i| {
        // Clone the Arc to increase the reference count
        let data = Arc::clone(&data);
        // Spawn a new thread
        thread::spawn(move || {
            // Lock the RwLock for writing to get access to the data
            let mut data = data.write().unwrap();
            // Modify the data at index i
            data[i] += 1;
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the modified vector
    println!("{:?}", data.read().unwrap());
}


/// Challenge 2 - Handle potential deaklocks in a more complex way
fn deadlock_handling() {
    let data1 = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data2 = Arc::new(Mutex::new(vec![4, 5, 6]));

    let handles: Vec<_> = (0..3).map(|i| {
        let data1 = Arc::clone(&data1);
        let data2 = Arc::clone(&data2);
        thread::spawn(move || {
            // Ensure consistent lock ordering
            let (mut lock1, mut lock2) = if i % 2 == 0 {
                (data1.lock().unwrap(), data2.lock().unwrap())
            } else {
                (data2.lock().unwrap(), data1.lock().unwrap())
            };
            // Modify the data
            lock1[i] += 1;
            lock2[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data1.lock().unwrap());
    println!("{:?}", data2.lock().unwrap());
}


/// Challenge 4 - Extending the code to use conditional variables to synchronize thread execution
fn conditional_synchronization() {
    let data = Arc::new((Mutex::new(vec![1, 2, 3]), Condvar::new()));
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*data;
            let mut data = lock.lock().unwrap();
            while data[i] == 1 {
                data = cvar.wait(data).unwrap();
            }
            data[i] += 1;
            cvar.notify_one();
        });
        handles.push(handle);
    }

    {
        let (lock, cvar) = &*data;
        let mut data = lock.lock().unwrap();
        for i in 0..3 {
            data[i] += 4;
        }
        cvar.notify_all();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let (lock, _) = &*data;
    println!("{:?}", lock.lock().unwrap());
}