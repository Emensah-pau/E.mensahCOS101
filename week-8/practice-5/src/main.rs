fn main() {
    let mut city:Vec<String> = Vec::new();

    println!("The city vector has elements {}",city.len());

    let mut input1 = String::new();
    println!("How many are your preferred cities");
    std::io::stdin().read_line(&mut input1).expect("");
    let city_num:i32 = input1.trim().parse().expect("");

    for count in 0..city_num 
    {
        println!("enter city {}",count+1);
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("");
        let city_new:String = input2.trim().parse().expect("");
        city.push(city_new);
    }
    print!("your preferred cities are:\n");
    let mut count=1;
    for i in city 
    {
        println!("{}, {}",count,i);
        count+=1;
    }
}
