use std::io;

fn main() {
    // Display the menu
    println!("Menu\t\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\t- N3,200");
    println!("F = Fried Rice & Chicken\t- N3,000");
    println!("A = Amala & Ewedu Soup\t\t- N2,500");
    println!("E = Eba & Egusi Soup\t\t- N2,000");
    println!("W = White Rice & Stew\t\t- N2,500");

    // Input the type of food
    println!("Enter the type of food (P/F/A/E/W): ");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read line");
    let food_type = food_type.trim();

    // Input the quantity
    println!("Enter the quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity: f32 = quantity.trim().parse().expect("Please enter a valid number");

    // Calculate total charges
    let total_charges = match food_type {
        "P" => 3200.0 * quantity,
        "F" => 3000.0 * quantity,
        "A" => 2500.0 * quantity,
        "E" => 2000.0 * quantity,
        "W" => 2500.0 * quantity,
        _ => {
            println!("Invalid food type entered!");
            return;
        }
    };

    // Apply discount if total order is greater than N10,000
    let discount_percentage = if total_charges > 10000.0 { 0.05 } else { 0.0 };
    let discount_amount = total_charges * discount_percentage;
    let final_amount = total_charges - discount_amount;

    // Display the total charges and discount
    println!("Total Charges: N{:.2}", total_charges);
    println!("Discount: N{:.2}", discount_amount);
    println!("Final Amount: N{:.2}", final_amount);
}
