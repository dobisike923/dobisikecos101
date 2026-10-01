fn main() {
    let fullname = "Chidubem Daniel Obisike";
    let department = "Computer Science";
    let uni = "Pan Atlantic University";


    let mut school = "School of science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
   // check lenght
    println!("The lenght of my fullname is: {}",fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}",school);
    println!("{}",uni);
}
