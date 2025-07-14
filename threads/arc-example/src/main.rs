use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create data on the heap, wrapped in an Arc.
    // `Arc` stands for "Atomically-Referenced Counter". It's a smart pointer
    // that allows multiple owners of the same data across threads. When the last
    // owner (the last Arc pointer) is dropped, the data is cleaned up.
    let shared_data = Arc::new(String::from("This is some shared data"));

    println!(
        "[Main] Initial reference count: {}",
        Arc::strong_count(&shared_data)
    );

    // 2. Clone the Arc for the new thread.
    // Cloning an Arc does not copy the data. It just creates a new pointer
    // to the same data and atomically increments the internal reference counter.
    // This is a very cheap operation.
    let data_for_thread = Arc::clone(&shared_data);

    println!(
        "[Main] Reference count after cloning: {}",
        Arc::strong_count(&shared_data)
    );

    // 3. Spawn a new thread and move the cloned Arc into it.
    let handle = thread::spawn(move || {
        // The new thread now has its own "owning" reference to the data.
        println!("[Thread] I have access to the data: '{}'", data_for_thread);
        thread::sleep(Duration::from_secs(1));
        println!("[Thread] Finished.");
        // `data_for_thread` is dropped here, decrementing the reference count.
    });

    // 4. Wait for the spawned thread to complete.
    handle.join().unwrap();

    println!(
        "[Main] Final reference count: {}",
        Arc::strong_count(&shared_data)
    );
}
