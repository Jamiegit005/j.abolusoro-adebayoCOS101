use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter value of n: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let n:i32 = input1.trim().parse().expect("Not a valid number");

    for a in 1..=n {
        for b in 1..=25 {
            let c = a * b;
            println!("{} x {} = {}",a,b,c);
        }
    }


}
