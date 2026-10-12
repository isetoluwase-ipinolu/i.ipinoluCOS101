use std::io;

fn main() {
        let aps_table = vec![
        // Office Admin Category
        ("APS 1-2", "Office Admin", "Intern"),
        ("APS 3-5", "Office Admin", "Administrator"),
        ("APS 5-8", "Office Admin", "Senior Administrator"),
        ("EL1 8-10", "Office Admin", "Office Manager"),
        ("EL2 10-13", "Office Admin", "Director"),
        ("SES", "Office Admin", "CEO"),

        // Academic Category
        ("APS 1-2", "Academic", "-"),
        ("APS 3-5", "Academic", "Research Assistant"),
        ("APS 5-8", "Academic", "PhD Candidate"),
        ("EL1 8-10", "Academic", "Post-Doc Researcher"),
        ("EL2 10-13", "Academic", "Senior Lecturer"),
        ("SES", "Academic", "Dean"),

        // Lawyer Category
        ("APS 1-2", "Lawyer", "Paralegal"),
        ("APS 3-5", "Lawyer", "Junior Associate"),
        ("APS 5-8", "Lawyer", "Associate"),
        ("EL1 8-10", "Lawyer", "Senior Associate 1-2"),
        ("EL2 10-13", "Lawyer", "Senior Associate 3-4"),
        ("SES", "Lawyer", "Partner"),

        // Teacher Category
        ("APS 1-2", "Teacher", "Placement"),
        ("APS 3-5", "Teacher", "Classroom Teacher"),
        ("APS 5-8", "Teacher", "Senior Teacher"),
        ("EL1 8-10", "Teacher", "Leading Teacher"),
        ("EL2 10-13", "Teacher", "Deputy Principal"),
        ("SES", "Teacher", "Principal"),
    ];

    println!("=== FEDERAL GOVERNMENT PUBLIC SERVICE APS LEVEL CHECKER ===");

    let categories_arr: [&str; 4] = ["Office Admin", "Academic", "Lawyer", "Teacher"];
    println!("Available categories are {:?}", categories_arr);

    let mut category_input = String::new();
    println!("\nEnter category:");
    io::stdin().read_line(&mut category_input).expect("Failed to read input");
    let category = category_input.trim();

    let mut choice_input = String::new();
    println!("How would you like to check your level?");
    println!("1. Check by Role (press 1)");
    println!("2. Check by Years of Experience (press 2)");
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice = choice_input.trim();

    if choice == "1" {
        let mut role_input = String::new();
        println!("Enter Staff Role (e.g., Intern, Associate, Senior Lecturer, Principal):");
        io::stdin().read_line(&mut role_input).expect("Failed to read line");
        let role = role_input.trim();

        let result = find_by_role(&aps_table, category, role);

        // Decision making to display output
    if result != "Not Found" {
        println!("Found! The level is: {}", result);
    } else {
        println!("Record not found.");
    }

    } else if choice == "2" {
        let mut years_input = String::new();
        println!("How many years of experience do you have:");
        io::stdin().read_line(&mut years_input).expect("Failed to read input");
        let years: u32 = years_input.trim().parse().expect("Invalid input. Please type in a valid number next time");

        // Find the APS level based on years
        let level = find_by_years(years);
        println!("\nBased on {} years of experience, your APS Level is: {}", years, level);

    } else {
        println!("Invalid option. Please choose either 1 or 2");
    }
}

// function to find level in tuple created
fn find_by_role(table: &[(&str, &str, &str)], target_category: &str, target_role: &str) -> String {
    for row in table {
        let (aps_level, category, role) = *row;

        if category == target_category && role == target_role {
            return aps_level.to_string();
        }
    }

    return "Not Found".to_string();
}

// function to find level by years of experience

fn find_by_years(years: u32) -> &'static str {
    if years >= 1 && years <= 2 {
        "APS 1-2"
    } else if years >= 3 && years <= 5 {
        "APS 3-5"
    } else if years >= 5 && years <= 8 {
        "APS 5-8"
    } else if years >= 8 && years <= 10 {
        "EL1 8-10"
    } else if years >= 10 && years <= 13 {
        "EL2 10-13"
    } else {
        "SES"
    }
}