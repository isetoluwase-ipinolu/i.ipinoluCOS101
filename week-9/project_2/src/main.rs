use std::io;
use std::io::Write;

fn main() {
    println!("====== PAU SMIS (Student Management Information System) Portal ======");
    
    // create a vector to store our tuples: (name, matric number, department, level)
    let mut students: Vec<(String, i32, String, i32)> = Vec::new();

    loop {
        // get the student's name
        println!("\nEnter student's name (or type 'done' to finish):");
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read name");
        let name = name_input.trim();

        // if they typed "done", exit the loop
        if name == "done" {
            break;
        }

        // get the student's matric number
        println!("Enter {}'s matriculation number:", name);
        let mut matric_input = String::new();
        io::stdin().read_line(&mut matric_input).expect("Failed to read matric");

        // convert text to a number with error handling
        let matric: i32 = match matric_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Oops! That's not a valid number. Let's try adding this student again.");
                continue;
            }
        };

        // get the student's department
        println!("\nEnter {}'s department:", name);
        let mut department_input = String::new();
        io::stdin().read_line(&mut department_input).expect("Failed to read department");
        let department = department_input.trim();

        // get the student's level
        println!("What level is {} in (type the level number only):", name);
        let mut level_input = String::new();
        io::stdin().read_line(&mut level_input).expect("Failed to read level");

        let level: i32 = match level_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Oops! That's not a valid number. Let's try adding this student again.");
                continue;
            }
        };

        // add the tuple into our vector
        students.push((name.to_string(), matric, department.to_string(), level));
    }

    // If no students were entered, exit
    if students.is_empty() {
        println!("\nNo students entered. Goodbye!");
        return;
    }

    let mut file = std::fs::File::create("PAU_SMIS_Portal_table.csv").expect("create failed");

    // Write the CSV header first (good practice for tables!)
    let header = "Name,Matric Number,Department,Level\n";
    file.write_all(header.as_bytes()).expect("write failed");

    for row in students {
        // Convert the i32 numbers to strings so we can concatenate them safely
        let line = row.0 + "," + &row.1.to_string() + "," + &row.2 + "," + &row.3.to_string() + "\n";
        file.write_all(line.as_bytes()).expect("write failed");
    }
    
    println!("\nYour table has been successfully created as 'PAU_SMIS_Portal_table.csv'.");
}