fn main() {
    let a_t=450_000.00;
    let q_t=2.00;

    // Toshiba
    let ts_t=a_t * q_t;
    println!("Total sales for Toshiba is {}", ts_t);
    println!("Total quantity sold is 2 pieces");

    let a_m=1_500_000.00;
    let q_m=1.00;

    // Mac
    let ts_m=a_m * q_m;
    println!("Total sales for Mac is {}", ts_m);
    println!("Total quantity sold is 1 piece");

    let a_hp=750_000.00;
    let q_hp=3.00;

    // HP
    let ts_hp=a_hp * q_hp;
    println!("Total sales for HP is {}", ts_hp);
    println!("Total quantity sold is 3 pieces");

    let a_d=2_850_000.00;
    let q_d=3.00;

    // Dell
    let ts_d=a_d * q_d;
    println!("Total sales for Dell is {}", ts_d);
    println!("Total quantity sold is 3 pieces");

    let a_a=250_000.00;
    let q_a=1.00;

    // Acer
    let ts_a=a_a * q_a;
    println!("Total sales for Acer is {}", ts_a);
    println!("Total quantity sold is 1 piece");

    let ts=ts_t + ts_m + ts_hp + ts_d + ts_a;
    println!("Total sales of P.M Okeke and Sons Ltd is {}", ts);
    println!("Total quantity sold by P.M Okeke and Sons is 10 pieces");

    let av= ts / 10.0;
    println!("Average sales of P.M Okeke and Sons is {}", av);

}