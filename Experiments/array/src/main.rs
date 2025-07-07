use std::io;

// Arrays are a fixed-size collection of elements of the same type.
// They are allocated on the stack, making them very fast.

// A function that works on an array of any size using const generics.
// `N` is a compile-time constant representing the array's length.
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

fn main() {
    // 1. Creation
    // Type is inferred: [i32; 5] (an array of 5 i32 integers)
    let a = [10, 20, 30, 40, 50];

    // Explicit type annotation
    let b: [char; 3] = ['a', 'b', 'c'];

    // Initialize an array with the same value for all elements.
    // This creates an array of 100 zeros.
    let c = [0; 100];
    println!("The 99th element of array 'c' is: {}", c[99]);

    // 2. Accessing Elements
    let first_element = a[0]; // Access the first element
    let second_element = a[1]; // Access the second element
    println!("\nElements of 'a': {} and {}", first_element, second_element);

    // 3. Iteration
    println!("\nIterating through array 'b':");
    for character in b { // .into_iter() is called by default, moving elements if not Copy
        println!("  Character: {}", character);
    }

    // 4. Slicing
    // A slice is a "view" into an array. It doesn't have ownership.
    // This creates a slice containing elements from index 1 up to (but not including) 4.
    let a_slice: &[i32] = &a[1..4]; // a_slice will be [20, 30, 40]
    println!("\nSlice of 'a': {:?}", a_slice);

    // 5. Useful Methods
    println!("\nArray 'a' has {} elements.", a.len());
    println!("Sum of 'a' is: {}", sum_array(a));

    // 6. Runtime Safety and Panics
    // Accessing an index that is out of bounds will cause a `panic`.
    // The following line, if uncommented, would crash the program:
    // let invalid_element = a[99];

    // Safe access with `.get()`
    // `.get()` returns an `Option<&T>`, which is `Some(&value)` if the index is valid,
    // and `None` if it's out of bounds. This prevents panics.
    println!("\n--- Safe Access Example ---");
    println!("Please enter an array index to access element from {:?}", a);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let index: usize = user_input.trim().parse().expect("Please type a number!");

    match a.get(index) {
        Some(element) => println!("The value at index {} is: {}", index, element),
        None => println!("Index {} is out of bounds for an array of length {}.", index, a.len()),
    }
}