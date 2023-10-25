fn main() {
    let p:f64 = 520_000_000.00;
    let r:f64 = 10.00;
    let n:f64 = 5.00;

    // compound interest
    let a = p * ( 1.0 + (r /100.0)).powf(n);
    println!("amount is {}", a);
    let ci = a - p;
    println!("compound interest is {}", ci);

}