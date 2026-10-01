fn main() {
    let n1 = "Electrical". to_string();
    let n2 = "Electronic". to_string();
    let n3 = "Engineering". to_string();
    let n4 = n1 + &n2 + &n3; // n2 and n3 refrence is passed

    // About electrical/Electronic    
    println!("\nThe {} is informed by the aspiration to train
        electrical/electronic Engineering professionals in the areas of design
        , building and maintenance of electrical control system,", n4);

    let w1 = "Computer". to_string();
    let w2 = "Science". to_string();
    let w3 = w1 + &w2; // w2 refrence is passed
    println!();
    println!("{} is aimed at developing competent, creative,
    innovative, enterprenual and ethically-minded persons,
    capable of creating value in the diverse fields of computer science. ",n3);

}
