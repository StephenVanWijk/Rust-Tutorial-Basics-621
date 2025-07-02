#![allow(dead_code)]

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

pub fn constants() -> (u32, f64, char){
    // Constant names are always in SCREAMING_SNAKE_CASE.
    const MAX_POINTS: u32 = 100_000; // Constants are immutable by default
    const PI: f64 = 3.14159;
    const FERRIS: char = '🦀';
    (MAX_POINTS, PI, FERRIS)
}

pub fn fixed_length_collection() -> [i32; 3] {
    let nums: [i32; 3] = [1, 2, 3];
    nums
}