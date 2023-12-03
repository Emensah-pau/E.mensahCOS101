use std::io;

fn main() {
    let mut name:Vec<String> = Vec::new();
    let mut age:Vec<i32> = Vec::new();
    let mut city:Vec<String> = Vec::new();
    let mut experience:Vec<i32> =Vec::new();

    println!("WELCOME TO ERNST & YOUNG(EY) GLOBAL LIMITED");
    println!("");

    println!("How many candidates are there for the interview");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("");
    let turn:i32 = input1.trim().parse().expect("");
    println!("");
    println!("");

    for i in 1..turn+1 
    {
        println!("");
        println!("");
        println!("What is the name of candidate {}",i);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("");
        let new_name = input2.trim().to_string();
        name.push(new_name);

        println!("What is the age of candidate {}",i);
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("");
        let new_age:i32 = input3.trim().parse().expect("");
        age.push(new_age);

        println!("What is the city of residence of candidate {}",i);
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("");
        let new_city = input4.trim().to_string();
        city.push(new_city);

        println!("How many years of experience does candidate {} have",i);
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("");
        let new_exp:i32 = input5.trim().parse().expect("");
        experience.push(new_exp);
        
        
    }
    let mut count=1;
    let mut roll=1;
    let mut share=1;
    let mut dice=1;


    println!("");
    println!("");
    for l in name
    {
        println!("The name of candidate {} is {}",count,l);
        count+=1;
    }
    println!("");
    
    for k in age 
    {
        println!("The age of candidate {} is {}",roll,k);
        roll+=1;
    }
    println!("");
    
    for j in city 
    {
        println!("The city of residence of candidate {} is {}",share,j);
        share+=1;
    }
    println!("");
    
    for h in experience
    {
        println!("The years of work experience of candidate {} is {} years",dice,h);
        dice+=1;
    }
    
}

