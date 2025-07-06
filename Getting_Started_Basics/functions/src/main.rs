fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

unsafe fn dangerous_operation() {
    // This code might do something unsafe, like dereference a raw pointer
    let mut num = 5;
    let r = &mut num as *mut i32; // Raw pointer
    *r = 10; // Dereferencing a raw pointer is unsafe
    println!("Num is now: {}", num);
}

fn main() {
    // Using a pointer to a function as an argument.
    let result = apply_operation(10, 3, subtract);
    println!("Result of subtraction: {}", result);

    // Anonymous function (closure) example.
    let factor = 2;
    let multiply_by_factor = |num: i32| num * factor; // `factor` is captured from the environment

    // Using the closure to multiply a number by the factor.
    // This is a common use case for closures in Rust.
    let result = multiply_by_factor(5);
    println!("Result: {}", result);

    // Using a closure to double the elements in a vector.
    // This is a common use case for closures in Rust.
    let numbers = vec![1, 2, 3];
    let doubled_numbers: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("Doubled numbers: {:?}", doubled_numbers);

    // Using a closure to filter elements in a vector.
    let filtered_numbers: Vec<i32> = numbers.iter().filter(|&n| n % 2 == 0).map(|&n| n * 2).collect();
    println!("Filtered and doubled numbers: {:?}", filtered_numbers);

    // Using unsafe code block
    unsafe {
        dangerous_operation();
    }

    println!("Factorial of 5: {}", factorial(5)); // Output: 120

}