use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // --- Shared Counter (using Arc<Mutex<T>>) ---
    // Arc enables multiple threads to "own" a pointer to the same data.
    // Mutex provides mutually exclusive access to the inner `u32` to prevent data races.
    let shared_counter = Arc::new(Mutex::new(0_u32));
    let mut counter_handles = vec![];

    println!("--- Mutex Example (Shared Counter) ---");
    for i in 0..5 {
        // Clone the Arc to give each thread its own "shared ownership" pointer.
        let counter_for_thread = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            println!("Thread {} trying to acquire counter lock...", i);
            // Acquire the lock. This call blocks if another thread holds the lock.
            // `unwrap()` is used for simplicity; in real code, handle errors.
            let mut num = counter_for_thread.lock().unwrap();
            *num += 1; // Mutate the shared data
            println!("Thread {} incremented counter to: {}", i, *num);
            thread::sleep(Duration::from_millis(50)); // Simulate some work
            // The `MutexGuard` (returned by `lock()`) is dropped here when `num` goes out of scope,
            // automatically releasing the lock.
        });
        counter_handles.push(handle);
    }

    // Wait for all counter threads to finish
    for handle in counter_handles {
        handle.join().unwrap();
    }

    // Access the final value from the main thread
    let final_counter_value = *shared_counter.lock().unwrap();
    println!("Final counter value: {}", final_counter_value);
    println!("");


    // --- Shared Configuration String (using Arc<RwLock<T>>) ---
    // RwLock allows multiple readers concurrently, but only one writer exclusively.
    let shared_config = Arc::new(RwLock::new(String::from("Initial Config")));
    let mut config_handles = vec![];

    println!("--- RwLock Example (Shared Configuration) ---");

    // Multiple Reader Threads
    for i in 0..3 {
        let config_for_thread = Arc::clone(&shared_config);
        let handle = thread::spawn(move || {
            println!("Reader Thread {} trying to acquire read lock...", i);
            // Acquire a read lock. Multiple read locks can be held concurrently.
            let config = config_for_thread.read().unwrap();
            println!("Reader Thread {}: Config = \"{}\"", i, *config);
            thread::sleep(Duration::from_millis(100)); // Simulate reading work
            // Read lock is released when `config` goes out of scope.
        });
        config_handles.push(handle);
    }

    // A Writer Thread (to demonstrate exclusive access)
    let config_writer_thread = Arc::clone(&shared_config);
    let writer_handle = thread::spawn(move || {
        println!("Writer Thread trying to acquire write lock...");
        // Acquire a write lock. This blocks until all read locks (and other write locks) are released.
        let mut config = config_writer_thread.write().unwrap();
        *config = String::from("Updated Config by Writer"); // Mutate the shared data
        println!("Writer Thread: Config updated to \"{}\"", *config);
        thread::sleep(Duration::from_millis(150)); // Simulate writing work
        // Write lock is released when `config` goes out of scope.
    });
    config_handles.push(writer_handle);


    // Another Reader Thread (will likely read the updated config)
    let config_late_reader_thread = Arc::clone(&shared_config);
    let late_reader_handle = thread::spawn(move || {
        println!("Late Reader Thread trying to acquire read lock...");
        let config = config_late_reader_thread.read().unwrap();
        println!("Late Reader Thread: Config = \"{}\"", *config);
    });
    config_handles.push(late_reader_handle);


    // Wait for all config threads to finish
    for handle in config_handles {
        handle.join().unwrap();
    }

    // Access the final config value from the main thread
    let final_config_value = shared_config.read().unwrap();
    println!("Final config value: \"{}\"", *final_config_value);
}