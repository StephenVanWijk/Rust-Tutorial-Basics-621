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

pub fn loop_labels(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
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

pub fn while_loop_countdown() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_loop_iteration() {
    let a = [10, 20, 30, 40, 50];

    println!("Iterating through an array:");
    for element in a {
        println!("the value is: {element}");
    }

    println!("\nIterating through a range (reversed):");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn if_let_while_let_demo() {
    println!("--- `if let` for concise control flow ---");
    // Using `match` can be verbose if you only care about one variant.
    // let config_max: Option<i32> = Some(3);
    let config_max: Option<i32> = None;
    match config_max {
        Some(max) => println!("(Using match) The maximum is configured to be {}", max),
        None => println!("(Using if let/else) No maximum was configured.") , // We have to handle the `None` case, even if we do nothing.
    }

    // `if let` is a much cleaner way to do the same thing.
    if let Some(max) = config_max {
        println!("(Using if let) The maximum is configured to be {}", max);
    }

    // You can also include an `else` block.
    let another_config: Option<i32> = None;
    if let Some(max) = another_config {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("(Using if let/else) No maximum was configured.");
    }

    println!("\n--- `while let` for conditional loops ---");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped from stack: {}", top);
    }
    println!("Stack is now empty.");
}