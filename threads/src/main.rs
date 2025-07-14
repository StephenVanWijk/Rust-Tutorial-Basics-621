use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;


fn thread_clone() {
    let a = Arc::new([1, 2, 3]);
    // Spawn a new thread, giving it a cloned Arc.
    let handle = thread::spawn({
        let a = a.clone();
        move || {
            // This code runs in the new thread.
            dbg!(a);
        }
    });
    // The main thread continues and prints its version of the Arc.
    dbg!(a);

    // Wait for the spawned thread to finish. Without this, main might exit
    // before the spawned thread gets to run.
    handle.join().unwrap();
}

/// Demonstrates a multi-threaded producer-consumer pattern that gracefully terminates.
/// The producer sends a finite number of items and then a `None` "poison pill"
/// to signal the consumer to shut down.
fn producer_consumer_example() {
    // The queue now holds `Option<i32>` to allow for a termination signal (None).
    // We need to specify the type here because the compiler can't infer it from an empty VecDeque.
    let queue = Mutex::new(VecDeque::<Option<i32>>::new());

    thread::scope(|s| {
        // Consuming thread
        let consumer_handle = s.spawn(|| loop {
            let item_option = queue.lock().unwrap().pop_front();
            if let Some(item) = item_option {
                match item {
                    // If we receive a value, process it.
                    Some(val) => {dbg!(val);},
                    // If we receive `None`, it's the signal to stop.
                    None => break,
                }
            } else {
                // If the queue is empty, park the thread until the producer unparks it.
                thread::park();
            }
        });

        // Producing thread (the main thread in this scope)
        for i in 0..5 {
            println!("Producing {}", i);
            queue.lock().unwrap().push_back(Some(i));
            consumer_handle.thread().unpark(); // Wake up the consumer
            thread::sleep(Duration::from_millis(500));
        }

        // After sending all items, send the `None` signal to terminate the consumer.
        println!("Producer finished. Sending termination signal.");
        queue.lock().unwrap().push_back(None);
        consumer_handle.thread().unpark();
    });
}

fn main() {
    thread_clone();
    println!("\n--- Running Producer-Consumer Example ---");
    producer_consumer_example();
}
