#![allow(unused)]
// Generics are a way to write code that is abstract over types. They allow you
// to write functions, structs, and enums that can work with many different
// concrete data types, reducing code duplication.

// 1. Generic Functions
// This function `largest` is generic over some type `T`.
// The `<T: PartialOrd>` part is a "trait bound", which means this function
// can be called with any type `T` as long as it implements the `PartialOrd`
// trait (which allows for comparison like `>`).

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 2. Generic Structs
// We can define structs to use a generic type parameter.
// This `Point<T>` struct can hold any type `T`.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// We can also use multiple generic type parameters.
#[derive(Debug)]
struct PointMulti<T, U> {
    x: T,
    y: U,
}

// 3. Generic Enums
// Enums can also be generic. The most famous examples are in the standard library:
// `Option<T>` can be `Some(T)` or `None`.
// `Result<T, E>` can be `Ok(T)` or `Err(E)`.

// 4. Generic Methods in `impl` blocks
// We can implement methods on generic structs.
impl<T> Point<T> {
    // This method returns a reference to the `x` field of type `&T`.
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also constrain the implementation to only certain generic types.
// This `impl` block and its `distance_from_origin` method will only exist
// for `Point<f64>` types.
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    println!("--- Generic Functions ---");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("\n--- Generic Structs ---");
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Integer point: {:?}, Float point: {:?}", integer_point, float_point);

    let mixed_point = PointMulti { x: 5, y: 4.0 };
    println!("Mixed type point: {:?}", mixed_point);

    println!("\n--- Generic Methods ---");
    println!("The x coordinate of the integer point is: {}", integer_point.x());

    // The following line would not compile because `distance_from_origin` is
    // only defined for `Point<f64>`.
    // println!("Distance for integer point: {}", integer_point.distance_from_origin());
    println!("Distance for float point: {}", float_point.distance_from_origin());

    // 5. Performance of Generics: Monomorphization
    // At compile time, Rust generates concrete implementations for each type
    // a generic function or struct is used with. For example, it creates a
    // `largest_i32` and a `largest_char` function. This means there is no
    // runtime cost for using generics. It's a zero-cost abstraction.
}