#![allow(unused)]
// An enum is a custom type that can be any one of several "variants".

// 1. Defining an Enum
// Each variant can optionally hold data. This enum represents different
// types of web events.
enum WebEvent {
    // A variant with no associated data
    PageLoad,
    PageUnload,
    // A variant that holds a single value (a tuple-like variant)
    KeyPress(char),
    // A variant that holds a String
    Paste(String),
    // A variant that holds an anonymous struct
    Click { x: i64, y: i64 },
}

// 2. Using Enums in Functions
// `match` is the primary way to interact with enums. It forces you to
// handle every possible variant, ensuring your code is exhaustive.
fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}.", x, y);
        }
    }
}

// 3. Enums with Methods (`impl`)
// Just like structs, you can define methods on enums.
#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quit message: No action."),
            Message::Echo(text) => println!("Echoing: {}", text),
            Message::Move { x, y } => println!("Moving to coordinates x: {}, y: {}", x, y),
        }
    }
}

// 4. The `Option<T>` Enum: Rust's Answer to Null
// The standard library provides `Option<T>`, which is so common it feels like
// part of the language. It's defined as:
// enum Option<T> { None, Some(T) }
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("--- Basic Enum Usage ---");
    let key_press = WebEvent::KeyPress('R');
    let paste = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect_event(key_press);
    inspect_event(paste);
    inspect_event(click);

    println!("\n--- Enums with Methods ---");
    let messages = [
        Message::Echo("hello world".to_string()),
        Message::Move { x: 10, y: 20 },
        Message::Quit,
    ];
    for msg in &messages {
        msg.process();
    }

    println!("\n--- The Option<T> Enum ---");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of `six` is: {:?}", six);
    println!("The value of `none` is: {:?}", none);

    println!("\n--- `if let` for Concise Matching ---");
    // Sometimes, a full `match` is verbose if you only care about one case.
    // `if let` lets you match a single pattern.
    if let Some(value) = six {
        println!("Got a value from `six`: {}", value);
    }
}