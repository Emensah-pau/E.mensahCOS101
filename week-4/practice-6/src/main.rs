fn main() {
    let n1 = "electrical".to_string();
    let n2 = "electronic".to_string();
    let n3 = "engineering".to_string();
    let n4 = n1 + &n2 + &n3;

    println!("/n {}",n4);

    let w1 = "computer".to_string();
    let w2 = "science".to_string();
    let w3 = w1 + &w2;
    println!();
    println!("{}",w3);
}
