use std::io

fn main() {
    println!("===SHAPE CALCULATOR===");
    println!("1. Trapezium - Area");
    println!("2. Rhombus - Area");
    println!("3. Parallelogram - Area");
    println!("4. Cube - Surface Area");
    println!("5. Cylinder - Volume");
    println!("Enter your choice:");

    let mut choice = String::new()
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input")

    let choice:u32 =
    choice.trim().parse().expect("invalid choice");

    match choice{
    1 => println!("You selected trapezium");
    2 => println!("You selected rhombus - Area");
    3 => println!("You selected parallelogram - Area");
    4 => println!("You selected cube - Surface Area");
    5 => println!("You selected cylinder - Volume");
    _ => println!("invalid choice:");
 
    }
    
}
