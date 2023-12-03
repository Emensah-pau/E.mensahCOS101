fn main() {
    let numbers = [1,2,3,4,5];
    println!("original array is = {:?}",numbers);

    let s1 = &numbers[1..3];
    println!("2nd and 3rd element sliced = {:?}",s1);

    let s2 = &numbers[..3];
    println!("index 0 to 3 sliced = {:?}",s2);

    let s3 = &numbers[2..];
    println!("index 2 to 4 sliced = {:?}",s3);

    let s4 = &numbers[..];
    println!("index 0 to 4 sliced = {:?}",s4);


}
