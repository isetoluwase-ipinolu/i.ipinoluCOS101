use std::io::Write;
use std::io;

fn main() {

    println!("====== PAU SMIS(Student Management Information System) Portal ======");
    
    // create a vector to store our tuples: (name, matric number, department, level)
    let mut students: Vec<(String, i32)> = Vec::new();

    loop {
        // get the student's name
        println!("\nEnter student's name (or type 'done' to finish):");
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read name");
        let name = name_input.trim();

        // Simple string comparison: if they typed "done", exit the loop
        if name == "done" {
            break;
        }

        // get the student's matric number
        println!("Enter years of experience for {}:", name);
        let mut matric_input = String::new();
        io::stdin().read_line(&mut matric_input).expect("Failed to read years");

        //convert text to a number with basic error handling
        let matric: i32 = match matric_input.trim().parse() {
            Ok(number) => number, // If it successfully turns into a number, use it
            Err(_) => {           // If the user typed letters instead of a number...
                println!("Oops! That's not a valid number. Let's try adding this candidate again.");
                continue;         // ...skip the rest and restart the loop
            }
        };

        println!("\nEnter student's name (or type 'done' to finish):");
        let mut department_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read name");
        
        let name = name_input.trim();

        // get the candidate's years of experience
        println!("Enter years of experience for {}:", name);
        let mut years_input = String::new();
        io::stdin()
            .read_line(&mut years_input)
            .expect("Failed to read years");

        let years: i32 = match years_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {          
                println!("Oops! That's not a valid number. Let's try adding this student again.");
                continue;         
            }
        };

        //add the tuple into our vector
        candidates.push((name.to_string(), years));
    }

    // If no students were entered, exit
    if candidates.is_empty() {
        println!("\nNo candidates entered. Goodbye!");
        return;
    }

    let mut file = std::fs::File::create("Nigerian_Breweries_Plc_drinks_table.csv").expect("create failed");

    for row in drinks_table {
        let line = row.0.to_string() + "," + row.1 + "," + row.2 + "\n";
        file.write_all(line.as_bytes()).expect("write failed");
    }
    println!("\nYour table has been successfully created.");
    
}
