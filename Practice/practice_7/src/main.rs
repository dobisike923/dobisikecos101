fn main() {
    //initialization of tuples with data types
    let datatypes_tuple: (&str, f32, u8) = ("Rust", 3.14, 100); 
    println!("Tuple content = {:?}", datatypes_tuple);

    //initialization of tuples without data types
    let no_datatypes_tuple = ("Rust", "fun", 100); 
    println!("Tuple content = {:?   }", no_datatypes_tuple);

    // accessing tuple element at index 0
    println!("Value at index 0 = {}", datatypes_tuple.0);

    // accessing tuple element at index 1
    println!("Value at index 0 = {}", datatypes_tuple.1);

    // accessing tuple element at index 2
    println!("Value at index 0 = {}", datatypes_tuple.2);
}