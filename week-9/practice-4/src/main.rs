use std::fs;

fn main() {
    fs::remove_file("text3.txt").expect("");
    println!("file have been succesfully deleted");
}
