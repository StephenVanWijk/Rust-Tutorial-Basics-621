use std::io;

fn main() {
    let a1 = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut zin = String::new();

    io::stdin()
        .read_line(&mut zin)
        .expect("Failed to read line");

    let index: usize = zin
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a1[index];

    println!("The value of the element at index {index} is: {element}");

    let b: [i32;5] = [0; 5]; // An array of 5 elements, all initialized to 0
    println!("Array b: {:?}", b);
    
    let a = [1, 2, 3];
    for (i, val) in a.iter().enumerate() {
        println!("a[{i}] = {val}");
    }
}
        
 