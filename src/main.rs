mod enc;
mod tsql;
use std::io;

fn main() {
    tsql::init_db();
    while (true) {
        println!("What do you want to do");
        println!("1) Get all of your passwords");
        println!("2) Insert a new password");
        println!("3) Generate a random password");
        println!("4) shutdown");
        //enc::encrypt();
        let mut data_01 = String::new();
        io::stdin().read_line(&mut data_01).unwrap(); //to get input from the user
        let data_01: i8 = data_01.trim().parse().unwrap();
        println!("{}", data_01);
        match data_01 {
            1 => println!("hey"),

            2 => println!("hey"),

            3 => println!("hey"),
        
            4 => break,

            _ => println!("pls retry"),
        }
    }
}
