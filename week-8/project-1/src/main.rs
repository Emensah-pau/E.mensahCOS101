use std::io;

fn main(){
    let level = vec!["APS 1-2","APS 3-5","APS 5-8","APS 8-10","EL2 10-13","SES"];
    let job = vec!["OFFICE ADMINISTRATOR","ACADEMIC","LAWYER","TEACHER"];
    let pos_office = vec!["INTERN","ADMINISTRATOR","SENIOR ADMINISTRATOR","OFFICE MANAGER","DIRECTOR","CEO"];
    let pos_academic = vec!["____","RESEARCH ASSISTANT","PhD CANDIDATE","POST-DOC RESEACHER","SENIOR LECTURER","DEAN"];
    let pos_lawyer = vec!["PARALEGAL","JUNIOR ASSOCIATE","ASSOCIATE","SENIOR ASSOCIATE 1-2","SENIOR ASSOCIATE 3-4","PARTNER"];
    let pos_teacher = vec!["PLACEMENT","CLASSROOM TEACHER","SENIOR TEACHER","LEADING TEACHER","DEPUTY TEACHER","PRINCIPAL"];

    println!("WELCOME TO THE PUBLIC SERVICE APS CHECKER FOR THE FEDRAL GOVERNMENT OF NIGERIA");
    println!("");
    println!("This program only check for {},{},{} and {}.",job[0],job[1],job[2],job[3]);
    println!("");
    
    println!("How many staffs are checking their levels");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("");
    let num:i32 = input1.trim().parse().expect("");

    for turns in 0..num {
        println!("WELCOME TO THE PUBLIC SERVICE APS CHECKER FOR THE FEDRAL GOVERNMENT OF NIGERIA");
        println!("");
        println!("This program only check for {},{},{} and {}.",job[0],job[1],job[2],job[3]);
        println!("");
        println!("What is your occupation\n
                  Enter 1 for {}\n
                  Enter 2 for {}\n
                  Enter 3 for {}\n
                  Enter 4 for {}.",job[0],job[1],job[2],job[3]);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("");
        let occu:i32 = input2.trim().parse().expect("");

        println!("How many years of experience do you have");
        let mut input4= String::new();
        io::stdin().read_line(&mut input4).expect("");
        let years:i32 = input4.trim().parse().expect("");

        if occu == 1 || occu == 2|| occu == 3 || occu == 4
        {
            println!("what is your position (IN CAPITAL LETTERS)");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("");
            let pos = input3.trim();

            if pos == pos_office[0] || pos == pos_academic[0] || pos == pos_lawyer[0] || pos == pos_teacher[0]
            {
                println!("Your APS level is APS 1-2");
            }
            if pos == pos_office[1] || pos == pos_academic[1] || pos == pos_lawyer[1] || pos == pos_teacher[1]
            {
                println!("Your APS level is APS 3-5");
            }
            if pos == pos_office[2] || pos == pos_academic[2] || pos == pos_lawyer[2] || pos == pos_teacher[2]
            {
                println!("Your APS level is APS 5-8");
            }
            if pos == pos_office[3] || pos == pos_academic[3] || pos == pos_lawyer[3] || pos == pos_teacher[3]
            {
                println!("Your APS level is EL1 8-10");
            }
            if pos == pos_office[4] || pos == pos_academic[4] || pos == pos_lawyer[4] || pos == pos_teacher[4]
            {
                println!("Your APS level is EL2 10-13");
            }
            if pos == pos_office[5] || pos == pos_academic[5] || pos == pos_lawyer[5] || pos == pos_teacher[5]
            {
                println!("Your APS level is SES");
            }

            println!("");
            println!("");
            println!("Press any key on the keyboard to continiue");
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("");
            let con = input5.trim();

            println!("");
            println!("");
        }
    }
}