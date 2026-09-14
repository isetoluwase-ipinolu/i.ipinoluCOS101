//Rust program to read height of a person and determine if person is average height, tall or short.

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height in centimeters: ");
    io::stdin().read_line(&mut input).expect("Not a valid string.");
    let height:f32 = input.trim().parse().expect("Not a valid number.");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are average height.");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall.");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are short, my friend.");
    }
    else
    {
        println!("Your height is abnormal.");
    }

}
