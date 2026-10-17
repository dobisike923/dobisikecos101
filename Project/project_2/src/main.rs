use std::io::{self,Write};

fn main() {
    let mut candidate:Vec<(String,u32)>=Vec::new();
    println!("Enter the number of candidate:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let count:usize = match input.trim().parse(){
        Ok(n) => n,
        Err(_) =>{
           println!("Invalid number of candidate.");
           return;

        }    
    };

    for i in 0..count {
        println!("Enter candidate {} name:", i + 1);
         let mut name = String::new();
         io::stdin().read_line(&mut name).unwrap();

         println!("Enter years of of programming experience");
         let mut years_input = String::new();
         io::stdin().read_line(&mut years_input).unwrap();

         let years: u32 = match 
         input.trim().parse(){
          Ok(n) => n,
        Err(_) =>{
           println!("Invalid years of experience.");
           return;

         } 
         };

         candidate.push((name.trim().to_string(),years)); 

    }
     if candidate.is_empty() {
         println!("No candidate were entered.");
           return;
     }

      let mut highest =  &candidate[0];  
      for candidate in &candidate {
        if candidate.1 > highest.1{
            highest=candidate;
        }
      }

      println!("\nCandidate with the highest experience:");
      println!("Name: {}", highest.0);
      println!("Years of experience: {}", highest.1);

}