#![allow(dead_code)]

fn annotated_() {
    // Variables can be type annotated.
    let _logical: bool = true;

    let _a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let _default_float   = 3.0; // `f64`
    let _default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut _inferred_type = 12; // Type i64 is inferred from another line
    _inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // _mutable = true;
    
    // Variables can be overwritten with shadowing.
    let _mutable: bool = true;
}


pub fn basic_type_conversion() -> (f64, i16, u32, u8) { 
    let x: i32 = 5;
    let y: f64 = x as f64; // Convert i32 to f64

    let z: u8 = 255;
    let w: i16 = z as i16; // Convert u8 to i16

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;

    let t = false;
    let u = t as u8;

    (y, w, c, u)
}
pub fn char_from_string() -> (char, Option<char>) {
    let greeting = "hello";
    let c1 = greeting.chars().next().unwrap(); // Get the first character from the string
    let c2: Option<char> = greeting.chars().nth(1000); // Get the second character from the string
    (c1, c2)
}

pub fn constants() -> (u32, f64, char){
    // Constant names are always in SCREAMING_SNAKE_CASE.
    const MAX_POINTS: u32 = 100_000; // Constants are immutable by default
    const PI: f64 = 3.14159;
    const FERRIS: char = '🦀';
    (MAX_POINTS, PI, FERRIS)
}

pub fn different_var_forms(){
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let d = 'r'; // unicode character
    let ferris = '🦀'; // also a unicode character
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, d, ferris, bv, t.0, t.1, sentence
    );
}

pub fn fixed_length_collection() -> [i32; 3] {
    let nums: [i32; 3] = [1, 2, 3];
    nums
}

pub fn literals_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    // println!("1 - 2 = {}", 1u32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

pub fn simple_var_example (){
    let x: i8 = 5;
    println!("Variable x: {}", x);    
}

pub fn simple_var_overflow_a(){
    let x: i8 = 127; // Maximum value for i8, because it is signed,
                     // it can hold values from -128 to 127, 
                     // because 0 is in the middle. 
    println!("Variable x before overflow: {}", x);
    let y: i8 = x + 1; // This will cause an overflow
    println!("Variable y after overflow: {}", y); // This will wrap around to -128
}

pub fn simple_var_overflow_b(){
    let mut x:i8 = 10;

    for _ in 0..10 {
        x = x + 100;
    }

    println!("x = {}", x);
}

