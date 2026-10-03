use std::io;

fn main() {
    println!("============= RESTAURANT MENU ============");
    println!("P - Poundo Yam / Edikaikong Soup  - ₦3,200");
    println!("F - Fried Rice & Chicken          - ₦3,000");
    println!("A - Amala & Ewedu Soup            - ₦2,500");
    println!("E - Eba & Egusi Soup              - ₦2,000");
    println!("W - White Rice & Stew             - ₦2,500");

    // Get food type
    println!("\nEnter food type (P/F/A/E/W):");

    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Get quantity
    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: i32 = quantity.trim().parse().expect("Please enter a number");

    // Determine price
    let price: i32 = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // Calculate total
    let total = price * quantity;

    println!("\nPrice per item: ₦{}", price);
    println!("Quantity: {}", quantity);
    println!("Total before discount: ₦{}", total);

    // Apply discount if total is greater than ₦10,000
    if total > 10_000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount (5%): ₦{}", discount);
        println!("Final total: ₦{}", final_total);
    } else {
        println!("No discount.");
        println!("Final total: ₦{}", total);
    }
}