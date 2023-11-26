use std::io;

fn add(a:i32, b:i32) {
    let sum = a + b;

    println!("SUM OF A and B = {}",sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("failed");
    let a:i32 = input1.trim().parse().expect("failed");

    let mut input2 = String::new();
    println!("Enter parameter for B:");
    io::stdin().read_line(&mut input2).expect("failed");
    let b:i32 = input2.trim().parse().expect("failed");

    // call add function with arguments
    add(a,b);
}
