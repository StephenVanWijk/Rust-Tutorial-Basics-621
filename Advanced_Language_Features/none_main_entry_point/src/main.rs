fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let result = apply_operation(10, 3, subtract);
    println!("Result of subtraction: {}", result);
}