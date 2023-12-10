use std::io::Write;

fn main() {

    let lager = vec!["\nLAGER","\n33-Exports","\nDesperados","\nGoldberg","\nHeineken","\nStar"];
    let stout = vec!["\n\nSTOUT","\nLegend","\nTurbo-king","\nWilliams"];
    let non = vec!["\n\nNON-ALCOHOLIC","\nMaltina","\nAmstel-malta","\nMalta-gold","\nFayrouz"];

    let mut file = std::fs::File::create("project-1_file.txt").expect("");
    file.write_all(b"WELCOME TO NIGERIAN BREWIES PLC").expect("");
    // file.write_all(b"HERE IS A LIST OF OUR HIGH QUALITY CATEGORIES OF DRINKS\n"
    //    .as_bytes()).expect("");             
      
   
    for i in lager 
    {
        
        file.write_all(i.as_bytes()).expect("");
    }
    for j in stout
    {
        println!("");
       
        file.write_all(j.as_bytes()).expect("");
    }
    for k in non
    {
        println!("");
      
        file.write_all(k.as_bytes()).expect("");
    }


    println!("!!!!!!!successful!!!!!!!!");
}
