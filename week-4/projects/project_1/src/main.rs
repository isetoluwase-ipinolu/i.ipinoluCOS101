use std::io;

fn main() {
    println!("What is the value of a?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("What is the value of b?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("What is the value of c?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to read input");

    let d:f32 = b*b - 4.0*a*c;

    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The roots are:");
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);

    } else if d == 0.0 {
        let x = -b / (2.0 * a);

        println!("The roots are equal:");
        println!("x = {}", x);
    } else {
        println!("The equation has no real roots.");
    }
}
