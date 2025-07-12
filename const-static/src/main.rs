// 20250711 1314CET SDvW:
// WhyDPE: I have asked Gemini to make me examples in where the const constants are initialized using const fns.
// This is a simple example of how to use const fns to initialize constants.
// The constants are initialized at compile time, and the const fns are used to compute their values.
// This is useful for cases where you want to have constants that are not just simple literals,
// but rather depend on some computation that can be done at compile time.

// WhatDPE: TransitionDPE: Rust: constant values initialized at compile time using const fns.

const BASE_CAPACITY: usize = 128;

// Define a constant for a growth factor.
const GROWTH_FACTOR: usize = 2;

// A `const fn` that calculates a size based on an input and a growth factor.
// It uses simple arithmetic and a conditional, all computable at compile time.
const fn calculate_dynamic_capacity(requested_size: usize) -> usize {
    if requested_size == 0 {
        return BASE_CAPACITY; // Return base if 0 requested
    }
    // Simple multiplication, safe to do at compile time.
    // This is "dynamic" in that the result depends on the input `requested_size`.
    requested_size * GROWTH_FACTOR
}

// A `const fn` that combines two string slices (which are 'static)
// at compile time.
// Note: This relies on 'const_concat_bytes' feature for actual string concatenation at compile time,
// but for simple cases, existing string literals are usually sufficient.
// For more complex string manipulation, usually, you'd be passing pre-existing const string literals.
// For demonstration, let's just pick one based on a const argument.
const fn get_status_message(is_ok: bool) -> &'static str {
    if is_ok {
        "Status: OK"
    } else {
        "Status: ERROR"
    }
}

// Another `const fn` demonstrating array initialization.
// This is somewhat dynamic as the content depends on the loop iteration,
// but still fully compile-time determined.
const fn create_filled_array(fill_value: u8, size: usize) -> [u8; 5] {
    let mut arr = [0; 5]; // Array must have a fixed size
    let mut i = 0;
    while i < arr.len() && i < size { // Loop bounds must be known at compile time
        arr[i] = fill_value;
        i += 1;
    }
    arr
}

// Global constants initialized using the const fns.
// These are the "dynamic" initializations.
const SMALL_BUFFER_SIZE: usize = calculate_dynamic_capacity(10);
const LARGE_BUFFER_SIZE: usize = calculate_dynamic_capacity(256);
const ERROR_MESSAGE: &'static str = get_status_message(false);
const SUCCESS_MESSAGE: &'static str = get_status_message(true);
const PADDING_ARRAY: [u8; 5] = create_filled_array(b'X', 5);
const PARTIAL_ARRAY: [u8; 5] = create_filled_array(b'A', 3); // size < array length

fn main() {
    println!("Base Capacity: {}", BASE_CAPACITY);
    println!("Small Buffer Size: {}", SMALL_BUFFER_SIZE); // Calculated as 10 * 2 = 20
    println!("Large Buffer Size: {}", LARGE_BUFFER_SIZE); // Calculated as 256 * 2 = 512
    println!("Error Message: {}", ERROR_MESSAGE);
    println!("Success Message: {}", SUCCESS_MESSAGE);
    println!("Padding Array: {:?}", PADDING_ARRAY); // [88, 88, 88, 88, 88] (b'X' is 88)
    println!("Partial Array: {:?}", PARTIAL_ARRAY); // [65, 65, 65, 0, 0] (b'A' is 65)

    // You can even call const fns directly in other const contexts or at runtime
    const ANOTHER_CONSTANT: usize = calculate_dynamic_capacity(50);
    println!("Another constant: {}", ANOTHER_CONSTANT); // Calculated as 50 * 2 = 100

    // Calling at runtime is also fine:
    let runtime_calc = calculate_dynamic_capacity(7);
    println!("Runtime calculation: {}", runtime_calc); // Calculated as 7 * 2 = 14
}