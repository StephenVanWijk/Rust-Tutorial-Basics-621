// Conditionals, loops library root
pub fn conditionals_divisible(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn if_in_let_statement(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

pub fn if_in_let_statement_type_error(){
    // let condition = true;
    // let number = 0;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");
}

pub fn returning_values_from_loops(){
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return value from the loop
        }
    };

    println!("The result is: {}", result);
}
