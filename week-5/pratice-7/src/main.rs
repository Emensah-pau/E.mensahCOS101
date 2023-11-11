fn main() {
    let mut count =0;

    for nom in 1..21 {
        if nom > 10 {
            println!("{:?}",nom);
             continue;
        }
        count +=1;
    }
    println!("the count values greater than 10 (between 1 and 20) is:{}",count);
    
}
