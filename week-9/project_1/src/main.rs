use std::fs::File;
use std::io::prelude::*;

fn main() {
    let categories = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    
    let file_path = "drink_categories.txt";

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    if let Err(e) = write_categories_to_file(&mut file, &categories) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Drink categories saved to {}", file_path);
    }
}

fn write_categories_to_file(file: &mut File, categories: &Vec<(&str, Vec<&str>)>) -> std::io::Result<()> {
    for (category, drinks) in categories {
        writeln!(file, "{}:", category)?;

        for drink in drinks {
            writeln!(file, "  - {}", drink)?;
        }

        writeln!(file)?;
    }

    Ok(())
}
