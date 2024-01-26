struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn new(brand: &str, price: u64) -> Laptop {
        Laptop {
            brand: String::from(brand),
            price,
        }
    }

    fn calculate_total_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let hp_laptop = Laptop::new("HP", 650_000);
    let ibm_laptop = Laptop::new("IBM", 755_000);
    let toshiba_laptop = Laptop::new("Toshiba", 550_000);
    let dell_laptop = Laptop::new("Dell", 850_000);

    let quantity_per_brand = 3;
    let total_cost_hp = hp_laptop.calculate_total_cost(quantity_per_brand);
    let total_cost_ibm = ibm_laptop.calculate_total_cost(quantity_per_brand);
    let total_cost_toshiba = toshiba_laptop.calculate_total_cost(quantity_per_brand);
    let total_cost_dell = dell_laptop.calculate_total_cost(quantity_per_brand);

    let overall_total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    println!("Total cost for {} HP laptops: {}", quantity_per_brand, total_cost_hp);
    println!("Total cost for {} IBM laptops: {}", quantity_per_brand, total_cost_ibm);
    println!("Total cost for {} Toshiba laptops: {}", quantity_per_brand, total_cost_toshiba);
    println!("Total cost for {} Dell laptops: {}", quantity_per_brand, total_cost_dell);
    println!("Overall total cost: {}", overall_total_cost);
}
