use std::io::Write;

fn main() {

    let drinks_table = vec![
        ("Larger", "Stout", "Non-Alcoholic"),
        ("33 Export", "Legend", "Maltina"),
        ("Desperados", "Turbo King", "Amstel Malta"),
        ("Golgberg", "Williams", "Malta Gold"),
        ("Gulder", "-", "Fayrouz"),
        ("Heineken", "-", "-"),
        ("Star", "-", "-"),
    ];

    let mut file = std::fs::File::create("Nigerian_Breweries_Plc_drinks_table.csv").expect("create failed");

    for row in drinks_table {
        let line = row.0.to_string() + "," + row.1 + "," + row.2 + "\n";
        file.write_all(line.as_bytes()).expect("write failed");
    }
    println!("\nYour table has been successfully created.");
    
}
