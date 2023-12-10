use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("text2.txt").expect("");
    file.write_all("\nHello Class".as_bytes()).expect("");
    file.write_all("\nThis is an appendage of documents."
       .as_bytes()).expect("");
    println!("file open success");
}
