use std::io;

fn main() {
    println!("What multiples do u need");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let a:u32 = input1.trim().parse().expect("");

    println!("How many multiples of {} do u need:",a);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("");
    let n:u32 = input2.trim().parse().expect("");

     for e in 1..a+1 {
          print!("");
        for f in 1..n+1 {
            let x = e * f;
            print!("{} x {} = {}\t", e, f, x);
            print!("");
        }
       println!("");
     }

}

