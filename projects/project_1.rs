fn main() {
	let p:f64 = 520000000.0;//principal
	let r:f64 = 10.0;//rate
	let n:f64 = 5.0;//number of times interest compounds

	//compound interest
	let a = p * (1.0 + (r/100.0)).powf(n);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("The compound interest is {}", ci);
}