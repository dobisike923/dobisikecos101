
use std::io::Write;

fn main() {
    let header = "Student name,Matric number,Department,Level\n";
    let row_1 = "Oluchi Mordi,ACC10211111,Accounting,300\n";
    let row_2 = "Adans Aliyu,ECO10110101,Economics,100\n";
    let row_3 = "Shania Bolade,CSC10328828,Computer,200\n";
    let row_4 = "Adekunle Gold,ERE11020202,Electrical,200\n";
    let row_5 = "Blanca Edemoh,MRE10202001,Electrical,200\n";

    let mut file = std::fs::File::create("data.csv").expect("create failed");
    file.write_all("PAU-SMIS\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");  
    file.write_all(row_1.as_bytes()).expect("write failed");  
    file.write_all(row_2.as_bytes()).expect("write failed"); 
    file.write_all(row_3.as_bytes()).expect("write failed"); 
    file.write_all(row_4.as_bytes()).expect("write failed"); 
    file.write_all(row_5.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}
