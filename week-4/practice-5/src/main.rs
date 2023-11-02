fn main() {
    let fullname = "Pan-Atlantic University";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("before trim");
    println!("length is {}", fullname.len());
    println!();
    println!("after a trim");
    println!("length is {}",fullname.trim().len());
}
