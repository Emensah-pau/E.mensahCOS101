use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you experienced:
              enter Y for yes
              enter N for no");
    io::stdin().read_line(&mut input1).expect("");
    let input1 = input1.trim();

    if input1 == "Y" {
        println!("Enter your age");
        io::stdin().read_line(&mut input2).expect("");
        let a:f32 = input2.trim().parse().expect(""); 
        
        if a >= 40.0{
            println!("Your incentive is N1,560,000")
        }
        if a > 30.0 && a < 40.0 {
            println!("Your incentive is N1,480,000")
        }
        if a <= 29.0 {
            println!("Your incentive is N1,300,000")
        }
    }
    if input1 == "N" {
        println!("Your incentive is N100,000")
    } else {
        
    }

}
