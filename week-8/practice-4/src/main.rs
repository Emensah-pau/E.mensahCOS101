fn main() {
    let name = vec!["mary","sam","sally","greg","ada","mark","june","ife"];

    let age = vec![16,17,19,22,20,21,18,23];

    println!("\nAge allocation:\n");

    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
