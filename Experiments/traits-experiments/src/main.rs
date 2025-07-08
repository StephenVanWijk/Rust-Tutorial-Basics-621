// 2050708 0936CET SDvW 

// A trait defines a set of methods that a type must implement, representing a shared behavior.
// It's Rust's primary way of achieving abstraction, similar to interfaces in other languages.
// Traits can be used to define shared behavior across different types, allowing for polymorphism.
// Traits can be implemented for any type, including structs, enums, and even primitive types.
// Traits can also have default method implementations, which can be overridden by the implementing type.

// Orphan Rule or Coherence Rule in Rust
// The orphan rule is a fundamental part of Rust's trait system that ensures coherence and 
// prevents conflicting implementations of traits for types. 
// A crucial rule in Rust is the "orphan rule" (or "coherence" rule). 
// You can only implement a trait for a type if either the trait or the type is defined 
// within your current crate. 
// This prevents conflicting implementations of traits for external types.
// In simpler terms:
// You can implement a trait for a type if:
// std::fmt::Display (foreign trait) for your MyStruct (local type).
// MyTrait (local trait) for Vec<T> (foreign type).
// MyTrait (local trait) for MyStruct (local type).
// You can not implement a trait for a type if:
// std::fmt::Display (foreign trait) for Vec<T> (foreign type).

// 1. Defining a Trait
// Here we define a `Summary` trait with one required method `summarize_author` and one
// method `summarize` that has a default implementation.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default implementation can call other methods from the same trait.
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 2. Implementing a Trait for a Type
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    // No default implementation for `summarize_author`, so we must provide our own.
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // We override the default `summarize` implementation for a more specific behavior.
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.author)
    }   
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // We override the default `summarize` implementation for a more specific behavior.
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 3. Traits as Parameters (Trait Bounds)
// This function accepts any type that implements the `Summary` trait.
// This is called "static dispatch" because the compiler generates specialized
// code for each concrete type this function is called with.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The above is syntactic sugar for this more verbose generic bound syntax:
// pub fn notify<T: Summary>(item: &T) { ... }

// 4. Returning Types that Implement Traits
// This function returns some type that implements `Summary`, but the caller
// doesn't know if it's a Tweet or something else. This is useful for hiding
// implementation details.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rust_lang"),
        content: String::from("Check out the new release notes!"),
    }
    // Note: You can only return a single concrete type from a function like this.
    // An if/else that returns a Tweet or a NewsArticle would not compile here.
}

// This function takes two string slices and returns one of them.
// The lifetime annotation `<'a>` is a generic parameter that lets us tell the
// compiler about the relationship between the lifetimes of the references.
//
// Here, we are telling Rust that the returned reference (`-> &'a str`) will be
// valid for a lifetime that is the shorter of the lifetimes of `s1` and `s2`.
// This prevents the returned reference from outliving the data it points to.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("gemini_assist"),
        content: String::from("Traits in Rust are incredibly powerful!"),
    };

    let article = NewsArticle {
        headline: String::from("Rust Voted Most-Loved Language Again!"),
        author: String::from("The Community"),
    };

    println!("--- Using Trait Methods ---");
    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    println!("\n--- Using Traits as Parameters ---");
    notify(&tweet);
    notify(&article);

    println!("\n--- Returning a Trait ---");
    let summary = returns_summarizable();
    println!("Got a summary: {}", summary.summarize());

    // 5. Trait Objects for Dynamic Dispatch
    // Trait objects allow for values of different concrete types to be handled
    // via a common interface. This is "dynamic dispatch".
    println!("\n--- Using Trait Objects (`dyn Trait`) ---");
    // We create a vector that holds "trait objects".
    // A trait object points to both an instance of a type and a table used to
    // look up trait methods on that type at runtime.
    // We must use a pointer, like `Box`, because the concrete types can have different sizes.
    let items_to_summarize: Vec<Box<dyn Summary>> = vec![
        Box::new(tweet),
        Box::new(article),
    ];

    for item in items_to_summarize {
        // Rust uses dynamic dispatch here to call the correct `summarize` method
        // for the concrete type (`Tweet` or `NewsArticle`) inside the Box.
        println!("{}", item.summarize());
    }

    // 6. Standalone Example: Lifetimes
    // This demonstrates a function with explicit lifetime parameters.
    println!("\n--- Standalone Example: Lifetimes ---");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    // Usage:
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result); // `result` is valid as long as both `string1` and `string2` are valid.
}