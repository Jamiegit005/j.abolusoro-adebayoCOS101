use std::io::Read;
use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Enter criteria: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let criteria = input1.trim();

    if criteria == "administrator" {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if criteria == "project manager" {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if criteria == "employee" {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if criteria == "customer" {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if criteria == "vendor" {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else {
        print!("Not a valid criteria");
    }
}