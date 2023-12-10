use std::io::Write;

fn main() {

    let announce="week-9 : Rust file input nd output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("");
    file.write_all("This is rust programming\n"
       .as_bytes()).expect("");
    file.write_all(announce.as_bytes()).expect("");
    file.write_all(dept.as_bytes()).expect("");
    
    println!("\nDAta written to file.");
}
