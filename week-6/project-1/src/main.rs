use std::io;

fn main() {
    let mut num = 0;
    while num <= 150{

     num = num + 1;

    println!("Welcome to the PAN - ATLANTIC UNIVERSITY voting portal");

    println!("Are u a class representative:
              Enter Y for yes & N for no");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let imput1 = input1.trim();
    

    println!("Enter your current level");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("");
    let b:f32 = input2.trim().parse().expect("");
    

    println!("What is your CGPA");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("");
    let a:f32 = input3.trim().parse().expect("");

    if imput1== "Y" && b>= 200.0 && a>= 4.0 {
       println!("Enter name:");
       let mut input4 = String::new();
       io::stdin().read_line(&mut input4).expect("");
       let imput4 = input4.trim();
       
       println!("Age:");
       let mut input5 = String::new();
       io::stdin().read_line(&mut input5).expect("");
       let imput5 = input5.trim();

       println!("Email:");
       let mut input6 = String::new();
       io::stdin().read_line(&mut input6).expect("");
       let imput6 = input6.trim();
       
       println!("Department:");
       let mut input7 = String::new();
       io::stdin().read_line(&mut input7).expect("");
       let imput7 = input7.trim();

       println!("State of origin:");
       let mut input8 = String::new();
       io::stdin().read_line(&mut input8).expect("");
       let imput8 = input8.trim();

       println!("Dear {}
                 Age: {}years old
                 Email address: {}
                 Department: {}
                 S.O.O: {}
                 YOU ARE ELIGIBLE TO VOTE", input4,input5,imput6,input7,input8);
    }
    else {
        println!("You are not eligible to vote")
    }
    println!("Press OK to continue");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("");
    let input9 = input9.trim();


}
}
