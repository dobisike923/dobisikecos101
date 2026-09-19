use std::io;


fn main() {
    let mut input = String::new();

    //read experience status
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let is_experienced = input.trim().to_lowercase() == "yes";
    input.clear();

    if is_experienced {
        //Read age
        println!("Enter the employee's age:",);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let age: u32 = input.trim().parse().expect("Please enter a valid age");
        
        if age >= 40 {
           println!("Annual incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual incentive: N1,300,000");
        } else {
            //Handles age 28 and 29 if not explicitly defined in criteria
            println!("No specific incentive criteria for age 28 to 29.");
        } 
    } else {
            println!("Annual incentive: N100,000");
        }
}
