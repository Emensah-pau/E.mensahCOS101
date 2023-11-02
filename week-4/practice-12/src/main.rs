// rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    //input name
    println!("\nPlease enter your name.");
    let mut name = String ::new();
        io::stdin()
        .read_line(&mut name)
        .expect("failed to read output");
    println!("your name is:{}",name);

    // input age
    println!("\nEnter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed to read output");
    let age :i32 = age.trim().parse().expect("input not an integer");
    println!("age is: {}", age);     
}
