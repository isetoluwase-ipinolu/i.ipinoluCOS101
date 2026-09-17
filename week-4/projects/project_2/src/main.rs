use std::io;

fn main() {
    println!("How old are you?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let age:u8 = input1.trim().parse().expect("Failed to read input");

    println!("Are you experienced? (true/false)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input.");
    let experience:bool = input2.trim().parse().expect("Please enter true or false.");

    if experience == false {
        println!("Sorry but without experience, your annual incentive is only N100,000.");
}   else if experience == true && age <= 28 {
        println!("Your annual incentive is N1,300,000.");
}   if experience == true && age >= 29 && age <= 39 {
        println!("Your annual incentive is N1,480,000.");
}   if experience == true && age >= 40 {
        println!("Your annual incentive is N1,560,000.");
    }
}
