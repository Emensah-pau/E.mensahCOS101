use std::io;

fn age() {
{
    println!("\nhow many siblings do you have");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let n:i32 = input1.trim().parse().expect("");

    for e in 1..n+1 {
        println!("\nenter info of sibling {}",e);

        println!("\ninput name of sibling {}",e);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("");
        let n1:&str = input2.trim();


        println!("\ninput age of sibling {}",e);
        let mut input3= String::new();
        io::stdin().read_line(&mut input3).expect("");
        let n2:i32 = input3.trim().parse().expect("");

        if n2 >= 18 
        {
            println!("is sibling {} married or single",e);
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("");
            let n3:&str = input4.trim();

            if n3 == "married" 
            {
                println!("how many children does sibling {} have",e);
                let mut input5 = String::new();
                io::stdin().read_line(&mut input5).expect("");
                let n4:i32 = input5.trim().parse().expect("");

                println!("what city does sibling {} reside",e);
                let mut input9= String::new();
                io::stdin().read_line(&mut input9).expect("");
                let n8:&str = input9.trim();
 
            }
            else if n3 == "single" 
            {
                println!("is sibling {} a student or worker",e);
                let mut input6 = String::new();
                io::stdin().read_line(&mut input6).expect("");
                let n5:&str = input6.trim();

                if n5 == "student" 
                {
                    println!("what university is sibling {} studying",e);
                    let mut input7 = String::new();
                    io::stdin().read_line(&mut input7).expect("");
                    let n6:&str = input7.trim();

                    println!("what course is sibling {} studying",e);
                    let mut input8 = String::new();
                    io::stdin().read_line(&mut input8).expect("");
                    let n7:&str = input8.trim();
        
                }
    
            }
        }
        else if n2 < 18 
        {
            println!("have sibling {} written WAEC",e);
            let mut input10 = String::new();
            io::stdin().read_line(&mut input10).expect("");
            let n9:&str = input10.trim();

            if n9 == "yes" 
            {
                println!("what secondary school did sibling {} study",e);
                let mut input11= String::new();
                io::stdin().read_line(&mut input11).expect("");
                let n10:&str = input11.trim();
            }
            else 
            {
                println!("what class is sibling {}",e);
                let mut input12 = String::new();
                io::stdin().read_line(&mut input12).expect("");
                let n11:&str = input12.trim();
            }
        }
      let sib_arr:[&str;n] = [{},n3];
      println!("names if siblings are {:?}",sib_arr);

    }
}
}


fn main() {
    println!("FAMILY INFORMATION");
    age();
}
