use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter distance of car : ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut d:f32 = input1.trim().parse().expect("Not a valid number");
   //changing to kilometer
    d = d * 1.60934;
    println!("The Distance in Kilometers is {} Km", d);

    println!("Enter time taken to cover distance :");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let s:f32 = d / t ;
    let area = s;

    println!("The speed of the car is: {} km/h \n ", area);
}
