fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A: {}",a);
    println!("Value of B: {}",b);

    let mut res = a>b;
    println!("A greater than B: {}",res);

    res = a<b;
    println!("A lesser than B: {}",res);
}
