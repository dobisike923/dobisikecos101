use std::io::{self,Write};

fn main() {// Sample data :replace these with the exact enteries
    //From your project table
    let professions=vec!["Lawyer","Teacher","Academic"];
    let job_titles = vec![
    "assosiate",
    "classroom teacher",
    "research assistant",
    ];
    let min_experience = vec![5,3,1];
    let max_experience = vec![8,5,2];
    let aps_level = vec!["APS5-6","APS3-4","APS1-2"];

    let mut profession = String::new();
    let mut job_titles = String::new();
    let mut years_input = String::new();

    print!("Enter professions:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut profession).unwrap();

    print!("Enter job titles");
     io::stdout().flush().unwrap();
    io::stdin().read_line(&mut job_titles).unwrap();

    print!("Enter years of experience");
     io::stdout().flush().unwrap();
    io::stdin().read_line(&mut years_input).unwrap();

    let profession =
    profession.trim().to_lowercase();
    let job_title= job_titles.trim().to_lowercase();

    let years:u32 = match years_input.trim().parse() {
        Ok(value) => value,
        Err(_) =>{
            println!("Invalid experience. Enter a whole number.");
            return;
        }
    };

    let mut found = false;

    for i in 0..professions.len(){
        if professions[i] == profession
        &&job_titles == job_titles
        {
            found = true;
            println!("Matching staff record found!");

            if years  >= min_experience[i] 
             && years <= max_experience[i]
             {
                println!("Validated APS level: {}", aps_level[i]);
             }else{
                println!("Expetience dosen't match this APS range");
             }

             break;

             
        }

    }


    if !found {
        println!("No matching staff record foundss");
    }
    
}
