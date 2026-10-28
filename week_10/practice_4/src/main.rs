fn main() {
    let v = vec![15, 25, 35, 45, 55];
   let v2 = print_vector(v);
    println!("{}",v2[0]);
}

fn print_vector(v: Vec<i32>)-> Vec<i32>{

    println!("Inside print_vector function {:?}",v);
    return v
}