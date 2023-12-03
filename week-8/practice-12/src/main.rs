fn main() {
    let mut colors = ["red","green","yellow","white"];

    println!("\nOriginal array = {:?}",colors);

    let s1 = &mut colors[1..3];

    println!("first slice = {:?}",s1);

    s1[1] = "purple";
    println!("changed slice = {:?}", s1);

}
