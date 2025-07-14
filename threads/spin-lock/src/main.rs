use std::sync::Arc;
use std::thread;

mod spinlock;
use spinlock::SpinLock;

fn spinlock_example() {
    let counter = Arc::new(SpinLock::new(0));
    let mut handles = vec![];

    println!("Spinning up 10 threads to increment a counter 100,000 times each...");

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                // The lock is acquired, the guard is created.
                let mut guard = counter_clone.lock();
                // The value is incremented via DerefMut.
                *guard += 1;
                // The guard is dropped at the end of the scope, releasing the lock.
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *counter.lock(); // Lock to read the final value.
    println!("Final count: {}", final_count);
    assert_eq!(final_count, 1_000_000);
}

fn main() {
    println!("\n--- Running SpinLock Example ---");
    spinlock_example();
}
