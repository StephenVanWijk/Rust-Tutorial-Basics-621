use variables_number_strings_and_bools:: fixed_length_collection;
use variables_number_strings_and_bools::char_from_string;
use variables_number_strings_and_bools::literals_operators;


fn main() {
    let collection = fixed_length_collection();
    dbg!("Collection: {:?}", collection);
    let (a, b) = char_from_string();
    println!("First char: {}, Second char: {:?}", a, b);
    literals_operators();
}    

// use variables_number_strings_and_bools::simple_var_example;
// use variables_number_strings_and_bools::simple_var_overflow_b;
// use variables_number_strings_and_bools::different_var_forms;
// use variables_number_strings_and_bools::basic_type_conversion;
// use variables_number_strings_and_bools::constants;

    // Call the function to demcaronstrate variable usage
    // simple_var_example();
    // simple_var_overflow_b();
    // different_var_forms();
    // let (a, b, c, d) = basic_type_conversion();
    // println!("Converted values: a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    // dbg!("Converted values: a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    // let (mx_points, pi, ferris) = constants();
    // println!("Max points: {}, PI: {}, Ferris: {}", mx_points, pi, ferris);
    // dbg!("Max points: {}, PI: {}, Ferris: {}", mx_points, pi, ferris);