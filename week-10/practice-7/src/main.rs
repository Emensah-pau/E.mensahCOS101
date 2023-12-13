use std::io;

struct Employee{
    name:String,
    company:String,
    age:u32
}

fn main() {
    let mut input1 = String::new();
    println!("What is your name");
    io::stdin().read_line(&mut input1).expect("");
    let new_name= input1.trim().to_string();

    let mut input2 = String::new();
    println!("What is the name of your company");
    io::stdin().read_line(&mut input2).expect("");
    let new_company= input2.trim().to_string();

    let mut input3 = String::new();
    println!("How old are you");
    io::stdin().read_line(&mut input3).expect("");
    let new_age= input3.trim().parse().expect("");

    println!("");
    println!("*********************************************");

    let emp1 = Employee{

        name:String::from(new_name),
        company:String::from(new_company),
        age:(new_age),
    };
    println!("Name = {}\n",emp1.name);
    println!("Company = {}\n",emp1.company);
    println!("Age = {}\n",emp1.age);
    
}
