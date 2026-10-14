

fn main() {

    // Create an empty vector "City"
    let new_city = String::from("lagos");
    // Print city vector
    println!("The City vector has element {}",city.len());
    // push new elements into
    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("invalid imput");
    for count in 0..city_num {
    let mut input2 = String::new();
    println!("Enter city {}", count+1);
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let city_num:i32 = input2.trim().parse().expect("invalid imput");
    city.push(new_city); 
    }
    print!("Your preferred cities are:\n");
    let count=1;
    // loop to iterate element in vector 
    for i in city
    {
        // iterating through 1 on the vector
        println!("{} {}", count,city[0]);
        count+=1;
    }
}
