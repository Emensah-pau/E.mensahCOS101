fn main() {

    let fullname = "chibudun john umeh";
    let department = "computer science";
    let uni = "Pan-Atlantic university";

    let mut school = "School of science".to_string();
    //push string
    school.push_str(" and technology");

    println!("My name is {}", fullname);
    // check length
    println!("the length of my full name is:{}", fullname.len());
    println!("I am a student of {}",department);
    println!("{}",school);
    println!("{}",uni)
}
