use std::io;

fn main() {

    println!("Enter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let lower_bound:i32 = input1.trim().parse().expect("");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("");
    let upper_bound:i32 = input2.trim().parse().expect("");

    for x in lower_bound..upper_bound{
        println!("count level is {}", x);
    }
}
