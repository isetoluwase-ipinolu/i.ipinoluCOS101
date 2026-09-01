fn main() {
	let tq = 2;
	let mq = 1;
	let hpq = 3;
	let dq = 3;
	let aq = 1;

	//to find total number of laptops bought
	let num = tq + mq + hpq + dq + aq;
	println!("Total number of laptops bought is {}", num);

	//to find sum of the sale
	let tp:f64 = 450000.00;
	let mp:f64 = 1500000.00;
	let hpp:f64 = 750000.00;
	let dp:f64 = 2850000.00;
	let ap:f64 = 250000.00;

	//add together
	let sum = tp + mp + hpp + dp + ap;
	println!("The total sale is {}", sum);

	//to get average
	let avg:f64 = sum/num as f64;
	println!("The average sale is {}", avg);


}