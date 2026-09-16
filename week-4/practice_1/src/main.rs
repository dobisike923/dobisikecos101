
// Rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent information management system!");

    // input name
    println!("\nPlease Enter your name.");
    let mut name = String::new();
          io::stdin()
          .read_line(&mut name)
          .expect("Failed to read input");
    println!("Your name is: {}",name);
    
    //input age
    println!("\n Enter your age.");
    let mut age = String::new();
           io::stdin().read_line(&mut age).expect("Failed to repeat input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);             
}
