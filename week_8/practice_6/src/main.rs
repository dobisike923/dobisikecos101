fn main() {
    let num:i32 = 5
    mutate_num_to_zero(&mut num);
    println!("The value of num is:{}", num);
}

fn mutate_num_to_zero(parum_num:&mut i32) {
    *parum_num = *parum_num*0; //de reference
     println!("parum_num value is:{}", param_num);
}

