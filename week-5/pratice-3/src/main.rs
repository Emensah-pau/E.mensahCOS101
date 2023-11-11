use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height:");
    io::stdin().read_line(&mut input).expect("");
    let height:f32 = input.trim().parse().expect("");

    if height >= 150.0 && height <= 170.0
    {
        println!("you are  of average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("you are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        ("you are a dwarf");
    }
    else
    {
        println!("abnomal height");
    }
}
