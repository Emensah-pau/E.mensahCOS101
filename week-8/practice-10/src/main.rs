fn main() {
    let b:(i32,bool,f32) = (30,true,4.9);
    print(b);
}
fn print(x:(i32,bool,f32)) {
    println!("inside print method");
    let (age,is_male,cgpa)= x;
    println!("age is {},is male? {}, cgpa is {}",age,is_male,cgpa);
}