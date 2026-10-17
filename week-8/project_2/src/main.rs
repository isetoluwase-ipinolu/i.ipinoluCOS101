use std::io;

fn main() {
    println!("=== EY Nigeria Developer Interview Portal ===");
    
    // Create a vector to store our tuples: (Name, Years)
    let mut candidates: Vec<(String, i32)> = Vec::new();

    loop {
        // Step 1: Get the candidate's name
        println!("\nEnter candidate name (or type 'done' to finish):");
        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read name");
        
        let name = name_input.trim();

        // Simple string comparison: if they typed "done", exit the loop
        if name == "done" {
            break;
        }

        // Step 2: Get the candidate's years of experience
        println!("Enter years of experience for {}:", name);
        let mut years_input = String::new();
        io::stdin()
            .read_line(&mut years_input)
            .expect("Failed to read years");

        // Step 3: Convert text to a number with basic error handling
        let years: i32 = match years_input.trim().parse() {
            Ok(number) => number, // If it successfully turns into a number, use it
            Err(_) => {           // If the user typed letters instead of a number...
                println!("Oops! That's not a valid number. Let's try adding this candidate again.");
                continue;         // ...skip the rest and restart the loop
            }
        };

        // Step 4: Add the tuple into our vector
        candidates.push((name.to_string(), years));
    }

    // If no candidates were entered, exit gracefully
    if candidates.is_empty() {
        println!("\nNo candidates entered. Goodbye!");
        return;
    }

    // Step 5: Find the candidate with the highest experience
    let mut top_candidate = &candidates[0].0;
    let mut highest_years = candidates[0].1;

    for candidate in candidates.iter() {
        let name = &candidate.0;
        let years = candidate.1;

        if years > highest_years {
            highest_years = years;
            top_candidate = name;
        }
    }

    // Step 6: Announce the winner
    println!("\n=== Final Results ===");
    println!("Congratulations to {}! They secured the position with {} years of programming experience.",top_candidate, highest_years);
}