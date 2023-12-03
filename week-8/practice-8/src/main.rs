fn main() {
    let mut mountain = ("everest",846,'d',true);
    println!("original tuple :{:?}",mountain);

    mountain.1= 564;
    mountain.3= false;

    println!("change tuple = {:?}",mountain);
}
