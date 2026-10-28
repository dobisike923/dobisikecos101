fn main() {
    let v = vec![100, 200, 300];
   borrow_vector(&v);
    println!("printing values from main() v[0]={}",v[0]);
    println!("****************************");
}

fn borrow_vector(v: &Vec<i32>)-> &Vec<i32>{

    println!("****************************");
    println!("Inside print_vector function {:?}",v);
    println!("----------------------------");
    return v
}