#![allow(unused)]

// Tuples are a great way to return multiple values from a function.
fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    let average = sum as f64 / count as f64;
    (sum, count, average)
}

fn main() {
    // 1. Creation: A tuple is a fixed-size, ordered list of values of potentially different types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_str: (&str, &str, &str) = ("Stephen", "van", "Wijk");

    // 2. Accessing Elements: You can access elements by their index using dot notation.
    println!("Access by index: {} {} {} ", tup.0, tup.1, tup.2);
    println!("Name: {} {} {}\n", tup_str.0, tup_str.1, tup_str.2);

    // 3. Destructing: This is the most idiomatic way to use tuples.
    // It unpacks the tuple's values into separate variables.
    let (name, title, location) = ("Ferris", "Mascot", "Rustacean Station");
    println!("{} is the {} of {}.\n", name, title, location);

    // 4. Function Return Values: A common use case.
    let numbers = [10, 20, 30, 40, 50];
    let (total, num_elements, avg) = calculate_stats(&numbers);
    println!("Stats for the numbers:");
    println!("Sum: {}, Count: {}, Average: {:.2}\n", total, num_elements, avg);

    // 5. The Unit Type: An empty tuple `()` is called the "unit type".
    // Functions that don't return a value implicitly return the unit type.
    let _unit_type = (); // This has both a value `()` and a type `()`

    // 6. Nested Tuples: You can nest tuples within tuples.
    // This is useful for grouping related data together. 
    let point = (0, 5);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("X-axis at {x}"),
        (0, y) => println!("Y-axis at {y}"),
        (x, y) => println!("Point at ({x}, {y})"),
    }

    // 7. Subpattern binding with `@`
    // You can bind a variable to a value while also matching it against a more complex pattern.
    // This is very useful in `match` expressions with tuples.
    println!("\n--- Subpattern binding with @ ---");
    let user = (101, "Alice", 30);

    match user {
        // Here, `age @ 18..=35` checks if the third element is in the range 18-35.
        // If it is, the value is bound to the `age` variable for use inside the block.
        (id, name, age @ 18..=35) => {
            println!("User {} (ID: {}) is a young adult, age {}.", name, id, age);
        }
        (id, name, age) => {
            println!("User {} (ID: {}) is age {}.", name, id, age);
        }
    }

    // 8. Multiple Subpattern Bindings in a Tuple
    // You can apply subpattern bindings to multiple elements in the same tuple pattern.
    println!("\n--- Multiple Subpattern Bindings in a Tuple ---");
    let point_in_zone = (5, 10);

    match point_in_zone {
        // Here, we bind `x` and `y` while also checking if BOTH are within their respective ranges.
        (x @ 1..=10, y @ 5..=15) => {
            println!("Point ({}, {}) is inside the designated zone.", x, y);
        }
        (x, y) => {
            println!("Point ({}, {}) is outside the zone.", x, y);
        }
    }

    // 9. Subpattern Binding with Structs
    // You can also use `@` when destructuring structs.
    println!("\n--- Subpattern Binding with Structs ---");
    struct Person {
        id: u32,
        age: u32,
    }

    let person = Person { id: 1, age: 25 };

    // 20250706 1725CET SDvW
    // You can use a pattern like @ b..=a, 
    // but with a crucial restriction: 
    // the boundaries of the range (a and b) 
    // must be compile-time constants, 
    // not regular variables defined with let.
    // So: pattern: <value> @ A..=B is possible,
    // but A and B must be compile-time constants.
    match person {
        // Destructure the struct and bind `age` to `person_age` while checking the range.
        Person { id, age: person_age @ 21..=65 } => {
            println!("Person {} is of working age: {}.", id, person_age);
        }
        Person { id, age } => {
            println!("Person {} is age {}.", id, age);
        }
    }

    // 10. Subpattern Binding with Enums
    // This is a very common and powerful use case, especially for error handling.
    println!("\n--- Subpattern Binding with Enums ---");
    enum ServerResponse {
        Ok,
        Error(u16),
    }

    let response = ServerResponse::Error(404);

    match response {
        ServerResponse::Ok => println!("Response is OK."),
        ServerResponse::Error(code @ 400..=499) => println!("Client Error: {}. Not found or bad request.", code),
        ServerResponse::Error(code @ 500..=599) => println!("Server Error: {}. Please try again later.", code),
        ServerResponse::Error(code) => println!("An unknown error occurred: {}.", code),
    }

    // 11. Binding and Destructuring a Struct
    // You can bind the entire struct to a variable while also destructuring its fields.
    println!("\n--- Binding and Destructuring a Struct ---");
    #[derive(Debug)] // Add Debug trait to allow printing the struct
    struct User {
        id: u32,
        name: String,
    }

    fn log_user(user: &User) {
        println!("Logging user activity: {:?}", user);
    }

    let user = User { id: 1, name: "Bob".to_string() };

    match user {
        // Bind the whole struct to `u` while checking the `id`.
        u @ User { id: 1, .. } => {
            println!("Matched user with ID 1.");
            // We can now use `u` to pass the whole struct to another function.
            log_user(&u);
        }
        User { id, name } => {
            println!("Other user: {} with ID {}", name, id);
        }
    }

    // 12. Binding and Destructuring an Enum Variant
    // This works for enum variants too, which is very powerful.
    println!("\n--- Binding and Destructuring an Enum Variant ---");
    #[derive(Debug)]
    enum Message {
        Quit,
        Write { text: String, urgent: bool },
    }

    let msg = Message::Write { text: "System critical".to_string(), urgent: true };

    // 20250706 1741CET SDvW
    // Gemini made a mistake here: but I confronted it with the error message.
    // The original code had an ownership conflict because `msg` was moved into the match arms
    // when it was matched against `Message::Write { text, urgent: true }`.
    // This caused an error when trying to use `msg` again in the second arm.
    // The solution is to match on a reference to `msg` instead, which allows us
    // to borrow it without moving it, thus resolving the ownership conflict.
    // We match on `&msg` to borrow it instead of moving it.
    // This resolves the ownership conflict and makes all arms consistent.
    match &msg {
        // Because we match on a reference, `m` becomes a `&Message`,
        // and `text` becomes a `&String` automatically (this is "match ergonomics").
        // The `ref` keyword is no longer needed.
        m @ Message::Write { text, urgent: true } => {
            println!("Processing URGENT message: '{}'", text);
            // We can use `m` to forward the original, complete message.
            println!("Forwarding entire message: {:?}", m);
        }
        Message::Write { text, .. } => println!("Processing standard message: '{}'", text), // `text` is now a &String
        Message::Quit => println!("Quit message received."),
    }
}