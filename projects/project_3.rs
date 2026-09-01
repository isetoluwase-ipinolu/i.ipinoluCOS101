fn main() {
	let mut tv:f64 = 210000.00;
	let rate:f64 = 5.0;

	//loop to calculate depreciation for the 3 years
	for _ in 0..3 {
	tv = tv - (rate / 100.0 * tv);
	}

	//to display current value after 3 years
	println!("The value of the TV after 3 years is {}", tv);
}