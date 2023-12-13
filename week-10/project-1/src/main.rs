struct Shop{
    hp:u32,
    imb:u32,
    tosh:u32,
    dell:u32,
}
impl Shop {
    fn new_hp(&self)->u32 {
        self.hp * 3
    }
    fn new_imb(&self)->u32 {
        self.imb * 3
    }

    fn new_tosh(&self)->u32 {
        self.tosh * 3
    }
    fn new_dell(&self)->u32 {
        self.dell * 3
    }
}
fn main() {
    
    let price = Shop {
        hp:650_000,
        imb:755_000,
        tosh:550_000,
        dell:850_000,  
    };
    let total:u32 = price.new_hp() + price.new_dell() + price.new_imb() + price.new_tosh();
    println!("The total price for HP is N{}\n",price.new_hp());
    println!("The total price for IMB is N{}\n",price.new_imb());
    println!("The total price for Toshiba is N{}\n",price.new_tosh());
    println!("The total price for Dell is N{}\n",price.new_dell());
    println!("The sum total price is N{}\n",total);
}