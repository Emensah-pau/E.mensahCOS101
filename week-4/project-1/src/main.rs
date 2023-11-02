use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter distance in miles:");
        io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    let kilo_1:f32 = a * 1.60934;

    println!("Enter time in hours:");
        io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    let s_1:f32 = kilo_1/b;
    println!("the speed at which the car travles in kilometer is: {}", s_1);

}
