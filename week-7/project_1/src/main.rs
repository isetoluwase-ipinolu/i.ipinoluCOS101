use std::io;

fn calculate_area_of_trapezium() -> f64 {
    let mut height = String::new();
    println!("What is the height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let h: f64 = height.trim().parse().expect("Invalid input");

    let mut base1 = String::new();
    println!("What is the length of the first base:");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let b1: f64 = base1.trim().parse().expect("Invalid input");

    let mut base2 = String::new();
    println!("What is the length of the second base:");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let b2: f64 = base2.trim().parse().expect("Invalid input");

    let area = (b1 + b2) / 2.0 * h;
    println!("The area of your trapezium is {:.2}", area);
    area
}

fn calculate_area_of_rhombus() -> f64 {
    let mut diagonal1 = String::new();
    println!("What is the length of the first diagonal:");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let d1: f64 = diagonal1.trim().parse().expect("Invalid input");

    let mut diagonal2 = String::new();
    println!("What is the length of the second diagonal:");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let d2: f64 = diagonal2.trim().parse().expect("Invalid input");

    let area = 0.5 * d1 * d2;
    println!("The area of your rhombus is {:.2}", area);
    area
}

fn calculate_area_of_parallelogram() -> f64 {
    let mut base = String::new();
    println!("What is the base of the parallelogram:");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let b: f64 = base.trim().parse().expect("Invalid input");

    let mut altitude = String::new();
    println!("What is the altitude of the parallelogram:");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let a: f64 = altitude.trim().parse().expect("Invalid input");

    let area = b * a;
    println!("The area of your parallelogram is {:.2}", area);
    area
}

fn calculate_surface_area_of_cube() -> f64 {
    let mut length = String::new();
    println!("What is the length of each side:");
    io::stdin().read_line(&mut length).expect("Failed to read input");
    let l: f64 = length.trim().parse().expect("Invalid input");

    let s_a: f64 = 6.0 * l * l;
    println!("The surface area of your cube is {:.2}", s_a);
    s_a
}

fn calculate_volume_of_cylinder() -> f64 {
    let mut radius = String::new();
    println!("What is the radius of the cylinder:");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let r: f64 = radius.trim().parse().expect("Invalid input");

    let mut height = String::new();
    println!("What is the height of the cylinder:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let h: f64 = height.trim().parse().expect("Invalid input");

    let mut pie = String::new();
    println!("What is the value of pi you want to use (e.g. 3.14159):");
    io::stdin().read_line(&mut pie).expect("Failed to read input");
    let pi: f64 = pie.trim().parse().expect("Invalid input");

    let volume = pi * r * r * h;
    println!("The volume of your cylinder is {:.2}", volume);
    volume
}

fn main() {
    println!("===========================================================");
    println!("                    SHAPE CALCULATOR                       ");
    println!("===========================================================");
    println!("Calculate the area or volume of your chosen geometric shape");

    let city_arr: [&str; 5] = ["Trapezium", "Rhombus", "Parallelogram", "Cube", "Cylinder"];
    println!("The shapes available are {:?}", city_arr);

    let mut shape_type = String::new();
    println!("\nWhat shape do you want to calculate:");
    io::stdin().read_line(&mut shape_type).expect("Failed to read input");
    let shape = shape_type.trim().to_lowercase();

    if shape == "trapezium" {
        calculate_area_of_trapezium();
    } else if shape == "rhombus" {
        calculate_area_of_rhombus();
    } else if shape == "parallelogram" {
        calculate_area_of_parallelogram();
    } else if shape == "cube" {
        calculate_surface_area_of_cube();
    } else if shape == "cylinder" {
        calculate_volume_of_cylinder();
    } else {
        println!("Sorry, we cannot calculate that yet :(");
    }
}