// Define a struct to represent a Minister
#[derive(Debug)]
struct Minister {
    serial_number: u32,
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Define the datasets as arrays
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
}
    // Create a vector to store Minister structs
    let mut ministers: Vec<Minister> = Vec::new();

    // Iterate over the datasets and create Minister structs
    for (sn, (&name, &ministry, &zone)) in commissioners.iter().zip(ministries.iter()).zip(geopolitical_zones.iter()).enumerate() {
        let minister = Minister {
            serial_number: (sn + 1) as u32,
            name: name.to_string(),
            ministry: ministry.to_string(),
            geopolitical_zone: zone.to_string(),
        };
        ministers.push(minister);
    }

    // Display the merged data
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEO. ZONE");
    for minister in
