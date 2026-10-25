
use std::io::Write;

fn main() {
    let header = "Comissioner,Ministry,Zone,\n";
    let row_1 = "Aigbogbun Alamba Dauda,Internal affairs,South west\n";
    let row_2 = "Murtala Afeez bendu,Justice,North east\n";
    let row_3 = "Okorocha Callistus Ogbona,Defense,South south\n";
    let row_4 = "Adewale Jimoh Akanbi,Power and steel,South west\n";
    let row_5 = "Osazuwa Faith Etieye,Petroleum,South east\n";

    let mut file = std::fs::File::create("data.csv").expect("create failed");
    file.write_all("The EFCC Merge\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");  
    file.write_all(row_1.as_bytes()).expect("write failed");  
    file.write_all(row_2.as_bytes()).expect("write failed"); 
    file.write_all(row_3.as_bytes()).expect("write failed"); 
    file.write_all(row_4.as_bytes()).expect("write failed"); 
    file.write_all(row_5.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}
