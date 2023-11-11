use std::io;

fn main() {
    println!("          MENU:                  PRICE:
              P = Poundo yam/Edinkaikan soup - N3,200 
              F = Fried rice & chicken       - N3,000
              A = Amala & Ewedu soup         - N2,500
              E = Eba & Egusi soup           - N2,000
              W = White rice & stew          - N2,500
              
              Promo-Promo-Promo: Buy above 10K and get a 10% discount!!!");
     

    // let P = "Poundo_yam/Edinkaikan_soup"; 
    // let F = "Fried_rice_&_chicken";
    // let A = "Amala_&_Ewedu_soup";
    // let E = "Eba_&_Egusi_soup";
    // let W = "White_rice_&_stew";


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    
    println!("how many portions of poundo yam and edikaikan soup do u want: ");
    io::stdin().read_line(&mut input1).expect("");
    let p:f32 = input1.trim().parse().expect("");
    let pt:f32 = 3200.0 * p;


    println!("How many portions of fried rice and chicken do u want:");
    io::stdin().read_line(&mut input2).expect("");
    let q:f32 = input2.trim().parse().expect("");
    let qt:f32 = 3000.0 * q;

    println!("How many portions of Amala and Ewedu soup do u want:");
    io::stdin().read_line(&mut input3).expect("");
    let a:f32 = input3.trim().parse().expect("");
    let at:f32 = 2500.0 * a;

    println!("How many portions of Eba and Egusi soup do u want:");
    io::stdin().read_line(&mut input4).expect("");
    let e:f32 = input4.trim().parse().expect("");
    let et:f32 = 2000.0 * e;

    println!("How many portions of White rice and stew do u want:");
    io::stdin().read_line(&mut input5).expect("");
    let w:f32 = input5.trim().parse().expect("");
    let wt:f32 = 2500.0 * w;

    let total:f32 = pt + qt + at + et + wt;
    println!("your total bill is N{}", total);

    if total >= 10000.0 {
        let pro:f32 = total * 0.9;
        println!("your bill after 10% discount is price is N{}",pro); 
    }

    println!("Enjoy your meal
             Thank u for patronizing us
             Hope to see you again");
}
