 use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your phone number: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let number:i64 = input3.trim().parse().expect("Not a valid number");

    println!("Enter your email address: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("Enter number of siblings: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let number:i64 = input5.trim().parse().expect("Not a valid number");

    println!("Enter number of children: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let number:i64 = input6.trim().parse().expect("Not a valid number");

    println!("Enter medical diagnosis: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    

    println!("Enter village of residence: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");

}

