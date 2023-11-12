use std::io;

fn main() {
    // Get user input for experience and age
    let experience = get_user_input("Enter the employee's experience (experienced or inexperienced):");
    let age = get_user_input("Enter the employee's age:");

    // Parse age as an integer
    let age: u32 = age.trim().parse().unwrap_or_else(|_| {
        println!("Invalid age. Please enter a valid number.");
        std::process::exit(1);
    });

    // Calculate incentive based on the given criteria
    let incentive = match experience.as_str() {
        "experienced" if age >= 40 => 1_560_000,
        "experienced" if age >= 30 && age < 40 => 1_480_000,
        "experienced" if age < 28 => 1_300_000,
        "inexperienced" => 100_000,
        _ => {
            println!("Invalid input for experience or age.");
            std::process::exit(1);
        }
    };

    // Display the calculated incentive
    println!("The annual incentive for the employee is: N{}", incentive);
}

// Helper function to get user input
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}
