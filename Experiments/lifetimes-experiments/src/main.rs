#![allow(unused)]
// Lifetimes are a way for the Rust compiler to reason about the scopes for which
// references are valid. Their goal is to prevent dangling references.

// 1. Lifetimes in Function Signatures
// This is the most common place you'll encounter explicit lifetimes.
// The syntax `<'a>` declares a generic lifetime parameter named 'a'.
// `x: &'a str` and `y: &'a str` mean both references must live at least as long as 'a'.
// `-> &'a str` means the returned reference is also tied to the lifetime 'a'.
// This tells the compiler that the returned reference will be valid as long as
// Both of the input references are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2. Lifetime Elision Rules
// In many common cases, you don't need to write lifetimes because the compiler
// can infer them based on a set of rules called "lifetime elision".
//
// Rule 1: Each parameter that is a reference gets its own lifetime.
//         `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
//
// Rule 2: If there is exactly one input lifetime, that lifetime is assigned
//         to all output lifetimes. `fn foo<'a>(x: &'a i32) -> &'a i32`
//
// Rule 3: If there are multiple input lifetimes, but one is `&self` or `&mut self`,
//         the lifetime of `self` is assigned to all output lifetimes.

// This function works because of elision rule #2.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 3. Lifetimes in Struct Definitions
// When a struct holds a reference, its definition needs a lifetime annotation.
// This ensures that an instance of `ImportantExcerpt` cannot outlive the
// reference it holds in its `part` field.
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods on structs with lifetimes also need annotations.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("--- Lifetimes in Functions ---");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    // This demonstrates the borrow checker in action.
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is '{}'", result2);
    } // `string4` goes out of scope here.
    // If we tried to use `result2` here, it would be a compile error because
    // its lifetime was tied to the shorter lifetime of `string4`.

    println!("\n--- Lifetimes in Structs ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // `i` is an instance of the struct that holds a reference to `first_sentence`.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The important excerpt is: {:?}", i);
    println!("Part from method: {}", i.announce_and_return_part("New excerpt!"));

    // 4. The 'static Lifetime
    // This special lifetime means the reference can live for the entire duration
    // of the program. All string literals have the 'static lifetime.
    let s: &'static str = "I have a static lifetime.";
    println!("\n--- Static Lifetime ---");
    println!("{}", s);
}