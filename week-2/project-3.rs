fn main() {
    let p:f64 = 210_000.00;
    let r:f64 = 5.00;
    let n:f64 = 3.00;

    // compound interest
    let a = p * (1.00 - (r / 100.0)).powf(n);
    println!("The value of the TV in the next 3 years is {}", a);
}