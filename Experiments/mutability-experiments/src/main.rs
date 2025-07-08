use std::cell::Cell;
use std::cell::RefCell;
// Mutability is about whether a value can be changed. In Rust, values are
// immutable by default, which is a core part of its safety guarantees.

// This function takes an immutable reference. It can read the string but not change it.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function takes a mutable reference. It can read AND change the string.
fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    // 1. Immutability by Default
    let x = 5;
    println!("The value of x is: {}", x);
    // The following line will not compile because `x` is immutable.
    // x = 6;

    // 2. Opting into Mutability with `mut`
    let mut y = 10;
    println!("\nThe initial value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // 3. Mutability and Borrows
    // The borrowing rules are the heart of Rust's safety model.
    // You can have EITHER:
    // - One mutable reference (`&mut T`) in a scope.
    // - Any number of immutable references (`&T`) in a scope.
    // ...but not both at the same time.

    let mut s = String::from("hello");
    println!("\nOriginal string: '{}'", s);

    // We can create an immutable borrow to read the length.
    let len = calculate_length(&s);
    println!("The length of the string is: {}", len);

    // We can create a mutable borrow to change the string.
    append_world(&mut s);
    println!("The modified string is: '{}'", s);

    // 4. The Borrow Checker in Action
    // This demonstrates the borrowing rules.
    let mut data = vec![1, 2, 3];

    // Create an immutable reference to the first element.
    let first_element = &data[0];

    // The following line will NOT compile. You cannot have a mutable borrow
    // of `data` while an immutable borrow (`first_element`) exists.
    // This prevents a situation where `data.push(4)` might reallocate memory,
    // invalidating the `first_element` reference.
    // data.push(4);

    println!("\nThe first element is: {}", first_element);
    // The immutable borrow `first_element` goes out of scope here, so now we can mutate `data`.
    data.push(4);
    println!("The modified data is: {:?}", data);

    // 5. Interior Mutability with `Cell`.
    // `Cell<T>` allows you to mutate data even when the `Cell` itself is
    let x = Cell::new(5); // x is immutable, but its interior can change
    x.set(10);
    println!("Cell value: {}", x.get()); // Output: Cell value: 10

    // 6. Interior Mutability with `RefCell`.
    // `RefCell<T>` allows for mutable borrows checked at runtime.
    // This is useful when you need to mutate data through an immutable reference.
    let s = RefCell::new(String::from("immutable reference, mutable interior"));
    // s is immutable, but we can get a mutable borrow of its interior
    {
        let mut borrowed_s = s.borrow_mut(); // Runtime borrow check
        borrowed_s.push_str("!");
    } // `borrowed_s` goes out of scope, releasing the mutable borrow
    println!("{}", s.borrow()); // Output: immutable reference, mutable interior!
}