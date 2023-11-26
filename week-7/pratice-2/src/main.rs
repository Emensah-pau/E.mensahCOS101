use std::io;

fn checker() {
    let mut input1 = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut input1).expect("failed");
    let ch:char = input1.trim().parse().expect("failed");

    if ch >= '0' && ch<= '9'
    {
        println!("character '{}' is a digit", ch);
    }
    else
    {
        println!("character '{}' is not a digit",ch);
    }
}



fn main() {
    println!("Welcome! This program checks whether a character variable contains a digit or not");
    checker()
}
