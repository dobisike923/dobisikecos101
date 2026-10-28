use std::io::Write;

fn main() {
    let header = "Brand,Quantity,Price each,\n";
    let row_1 = "HP,10,N650_000\n";
    let row_2 = "IBM,6,N755_000\n";
    let row_3 = "Toshiba,10,N550_000\n";
    let row_4 = "Dell,4,N850_000\n";
    
    let mut file = std::fs::File::create("data.csv").expect("create failed");
    file.write_all("Alaba Electronics\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");  
    file.write_all(row_1.as_bytes()).expect("write failed");  
    file.write_all(row_2.as_bytes()).expect("write failed"); 
    file.write_all(row_3.as_bytes()).expect("write failed"); 
    file.write_all(row_4.as_bytes()).expect("write failed"); 
    println!("\nData written to file.");
}