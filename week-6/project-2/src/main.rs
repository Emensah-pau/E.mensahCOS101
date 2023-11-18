use std::io;

fn main() {
    let mut num = 0;
    while num <= 500 {

    num = num + 1;

    println!("HELLO!!!
    WELCOME TO THE NIGERIAN RESEARCHERS GUIDE (NRG)");

    println!("PLEASE CAN YOU ENTER YOUR NAME:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let imput1 = input1.trim();

    println!("HOW MANY PAPERS HAVE YOU PUBLISHED PLEASE");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("");
    let a:f32 = input2.trim().parse().expect("");

    if a >=3.0 && a<=5.0 {
        println!("Dear {}
        Your incentive is N500,000",input1);
    }
    else if a >5.0 && a< 10.0 {
        println!("Dear {}
        Your incentive is N800,000",input1);
    }
    else if a >= 11.0 {
        println!("Dear {}
        Your incentive is N1,000,000",input1);
    }
    else if a < 3.0 {
        println!("Dear {}
        Your incentive is 100,000",input1)
    }
}
}
