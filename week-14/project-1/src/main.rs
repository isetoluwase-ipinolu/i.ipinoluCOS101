use std::io;
use std::io::Read;

fn main() {
    println!("1. Administrator");
    println!("2. Project manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut input = String::new();
    println!("\nChoose a number based on your status in Globacom (1/2/3/4/5):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let status: i32 = input.trim().parse().unwrap();

    if status == 1 {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    } else if status == 2 {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    } else if status == 3 {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    } else if status == 4 {
        let mut file = std::fs::File::open("customers_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    } else if status == 5 {
        let mut file = std::fs::File::open("data_plan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    } else {
        println!("Sorry, that is invalid. Please try again");
    }
}
