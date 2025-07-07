#![allow(unused)]
// A struct is a custom data type that lets you package together and name
// multiple related values that make up a meaningful group.

// To make our structs printable for debugging, we can derive the `Debug` trait.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 2. Tuple structs are useful when you want to give a whole tuple a name
// and make it a different type from other tuples.
#[derive(Debug)]
struct Color(i32, i32, i32); // An RGB color
#[derive(Debug)]
struct Point(i32, i32, i32); // A point in 3D space

// 3. Unit-like structs have no fields. They are useful in situations where you
// need to implement a trait on some type but don't have any data to store.
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // 1. Creating an Instance of a Classic Struct
    // We create an instance by stating the struct name and then providing
    // key: value pairs for all the fields.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing and modifying fields using dot notation
    println!("User {}'s email is {}.", user1.username, user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("User {}'s new email is {}.\n", user1.username, user1.email);

    // Using the `build_user` function to create an instance
    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("Built user2: {:?}\n", user2);

    // Struct Update Syntax: create a new instance from an old one
    // The `..user1` must come last. It specifies that any remaining fields
    // should get their values from the corresponding fields in `user1`.
    // This moves the `username` and `email` strings from user1, so user1
    // can no longer be used as a whole.
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2 // Note: user2 is moved here because its String fields are moved.
    };
    println!("User3 created with update syntax: {:?}\n", user3);
    // println!("{:?}", user2); // This would fail because user2.username was moved to user3

    // Creating instances of tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("The color is: {:?}, and the point is: {:?}", black, origin);
    println!("The first value of the color is: {}\n", black.0);

    // 4. Methods and Associated Functions
    // Methods are defined within an `impl` (implementation) block.
    let rect = Rectangle::new(30, 50);
    println!("The rectangle is: {:?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("The perimeter of the rectangle is {}.", rect.perimeter());

    let rect2 = Rectangle::new(10, 40);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    // Using an associated function (like a static method) to create a square
    let sq = Rectangle::square(25);
    println!("Created a square: {:?}", sq);
}

// A helper function to show struct instantiation
fn build_user(email: String, username: String) -> User {
    // Using the "field init shorthand" syntax because the parameter names
    // are the same as the struct field names.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// `impl` block is where you define methods and associated functions for a struct.
impl Rectangle {
    // This is an associated function that acts as a constructor.
    // It's common to use `new` for this purpose.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // This is a method. The first parameter is always `self`, which represents
    // the instance the method is being called on. `&self` is a borrow.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Here is another method to calculate the perimeter.
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an "associated function", not a method, because it doesn't take `self`.
    // You call it with `::` syntax, e.g., `Rectangle::square(3)`.
    // These are often used as constructors.
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}