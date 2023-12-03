fn main() {
    let b:(i32,bool,f64)=(110,false,10.0);
    print(b);
}

fn print(x:(i32,bool,f64)) {
    println!("inside print method");
    println!("{:?}",x);
}
