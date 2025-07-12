use std::thread;
use std::sync::Arc;

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
fn main() {
    thread_clone();
}
