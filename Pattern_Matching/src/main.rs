#![allow(unused)]
use std::thread;

fn match_example(val: &Option<i32>) -> String {
    match val {
        Some(x @ 0..=9) => format!("Digit: {}", x),
        Some(x) if *x < 0 => "Negative".into(),
        Some(_) => "Large number".into(),
        None => "Nothing".into(),
    }
}

fn scope_locality(){
    /*
    20250711 1117CET SDvW:
    Here, numbers is a local variable to the main function 
    (or whatever function contains this code). 
    It's not a global variable, 
    nor is it created inside the thread::scope closure 
    or the spawned thread closures.*/
    let numbers = vec![1, 2, 3]; // 'numbers' is the local variable
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len()); // 'numbers' is borrowed here
        });
        s.spawn(|| {
            for n in &numbers { // 'numbers' is borrowed here
                println!("{n}");
            }
        });
        // When the scope ends, all threads that haven’t been joined yet are automatically joined.
    });
}

fn main() {
    // Call the method on the struct instance.
    println!("{}", match_example(&Some(5)));
    println!("{}", match_example(&Some(-3)));
    println!("{}", match_example(&Some(15)));
    println!("{}", match_example(&None));
}
