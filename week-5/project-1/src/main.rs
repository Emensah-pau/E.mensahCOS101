use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    
    println!("Input value of A");
    io::stdin().read_line(&mut input1).expect("");
    let a:f32 = input1.trim().parse().expect("");

    println!("Input value of B");
    io::stdin().read_line(&mut input2).expect("");
    let b:f32 = input2.trim().parse().expect("");

    println!("Input value of C");
    io::stdin().read_line(&mut input3).expect("");
    let c:f32 = input3.trim().parse().expect("");

    let d = b.powf(2.0) + 4.0 * a * c;
    if d < 0.0 {
        println!("there are no real root")
    }
    else if d == 0.0 {
        println!("there is only one real root")
    }
    else if d > 0.0 {
        println!("there are two real roots")
    }

    let al = (-b + d.sqrt()) / 2.0 * a;
    let ml = (-b + d.sqrt()) / 2.0 * a;

    println!("the roots of the Equation are {} and {}", al,ml);

    
}
