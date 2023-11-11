use std::io;

fn main() {

    println!("Enter number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let mut num:i32 = input1.trim().parse().expect("");

    while num < 10 {
        println!("inside loop number value is {}", num);
        num+=1;
    }
    println!("outside loop number of value is {}", num);
}
