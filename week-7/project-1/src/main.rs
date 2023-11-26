use std::io;

fn trapezium() {
    println!("Enter height");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("");
    let t1:f32= input2.trim().parse().expect("");

    println!("Enter base1");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("");
    let t2:f32= input3.trim().parse().expect("");

    println!("Enter base2");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("");
    let t3:f32= input4.trim().parse().expect("");

    let tt= t1/2.0 * (t2 + t3);
    println!("the area of the trapezium is {}",tt)

}

fn rhombus() {
    println!("Enter diagonal-1");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("");
    let r1:f32= input5.trim().parse().expect("");

    println!("Enter diagonal-2");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("");
    let r2:f32= input6.trim().parse().expect("");

  
    let rr= 0.5*r1*r2;
    println!("the area of the rhombus is {}",rr)

}

fn parallelogram() {
    
    println!("Enter altitude");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("");
    let p1:f32= input7.trim().parse().expect("");

    println!("Enter base");
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("");
    let p2:f32= input8.trim().parse().expect("");

    let pp= p1 * p2;
    println!("the area of the parallelogram is {}",pp)

}

fn cube() {
    println!("Enter height");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("");
    let c1:f32= input9.trim().parse().expect("");

   
    let cc= 6.0 * c1.powf(2.0);
    println!("the area of the cube is {}",cc);

}

fn cylinder() {
    println!("Enter radius");
    let mut input10 = String::new();
    io::stdin().read_line(&mut input10).expect("");
    let c21:f32= input10.trim().parse().expect("");

    println!("Enter height");
    let mut input11 = String::new();
    io::stdin().read_line(&mut input11).expect("");
    let c22:f32= input11.trim().parse().expect("");

    let cc2= 3.149 * c21.powf(2.0) * c22;
    println!("the volume of cylinder is {}",cc2)

}

fn main() {
   loop {
    println!("HELLO AND WELCOME
    PLEASE WHAT CALCULATION WOULD I LIKE TO DO:
    ENTER t FOR AREA TRAPEZIUM
    ENTER r FOR AREA RHOMBUS
    ENTER p FOR AREA PARALLELOGRAM
    ENTER c FOR AREA CUBE
    ENTER c2 FOR VOLUME CYLINDER");


    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let input1:&str= input1.trim();

    if input1 == "t" 
    {
        trapezium();
    }
    else if input1 == "r" 
    {
      rhombus();
    }
    else if input1 == "p" 
    {
        parallelogram();
    }
    else if input1 == "c" 
    {
        cube();
    }
    else if input1 == "c2" 
    {
        cylinder();
    }

    println!("do u want to perform any calculation again
    Enter y for yes 
    Enter n for no");
    let mut input12 = String::new();
    io::stdin().read_line(&mut input12).expect("");
    let c21:&str = input12.trim();

    if input12 == "n" 
    {
        break
    }
}

}
