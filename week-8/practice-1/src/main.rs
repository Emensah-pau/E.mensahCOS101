fn main() {
    let v:Vec<i64> = Vec::new();
    println!("\nThe length of vec::new is : {}",v.len());

    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];
    println!("\nThe lenght of vec macro is:{}",v.len());
}
