use std::io::Write;
use std::io;

fn main() {

    let mut name: Vec<String> = Vec::new();
    let mut matric: Vec<String> = Vec::new();
    let mut dept: Vec<String> = Vec::new();
    let mut level: Vec<String> = Vec::new();

    println!("what is the number of students:");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("");
    let turns:u8 = input5.trim().parse().expect("");
 
    for i in 0..turns 
    {
       println!("");
       println!("Input name");
       let mut input1 = String::new();
       io::stdin().read_line(&mut input1).expect("");
       let new_name = input1.trim().to_string();
       name.push(new_name);

       println!("Input matric number");
       let mut input2 = String::new();
       io::stdin().read_line(&mut input2).expect("");
       let new_matric = input2.trim().to_string();
       matric.push(new_matric);

       println!("Input department");
       let mut input3 = String::new();
       io::stdin().read_line(&mut input3).expect("");
       let new_dept = input3.trim().to_string();
       dept.push(new_dept);

       println!("Input level");
       let mut input4 = String::new();
       io::stdin().read_line(&mut input4).expect("");
       let new_level = input4.trim().to_string();
       level.push(new_level);
    }

    let mut file = std::fs::File::create("project_2.txt").expect("");
    file.write_all("PAU-SIMS\n".as_bytes()).expect("");
    file.write_all("\n".as_bytes()).expect("");
    file.write_all("NAME".as_bytes()).expect("");
    file.write_all("\t".as_bytes()).expect("");
    file.write_all("\tMATRIC NUMBER".as_bytes()).expect("");
    file.write_all("\t".as_bytes()).expect("");
    file.write_all("\tDEPARTMENT".as_bytes()).expect("");
    file.write_all("\t".as_bytes()).expect("");
    file.write_all("\tLEVEL\n".as_bytes()).expect("");
    for j in name
    {
        let mut name = j + &String::from("\n");
        file.write_all(name.as_bytes()).expect("");
    }
    file.write_all("\t\t".as_bytes()).expect("");
    for k in matric
    {
        // let mut matric = k.clone() + &String::from("\t\t");
        let mut matric = k + &String::from("\n");
        file.write_all(matric.as_bytes()).expect("");
    }
    file.write_all("\t\t".as_bytes()).expect("");
    for l in dept
    {
        // let mut dept = l.clone() + &String::from("\t\t");
        let mut dept = l + &String::from("\n");
        file.write_all(dept.as_bytes()).expect("");
    }
    file.write_all("\t\t".as_bytes()).expect("");
    for m in level
    {
        // let mut level = m.clone() + &String::from("\t\t");
        let mut level = m + &String::from("\n");
        file.write_all(level.as_bytes()).expect("");
    }
    println!("");
    println!("File created");
    println!("File saved");
    println!("!!!!!SUCCESSFUL!!!!!");
}
