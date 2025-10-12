 fn main() {
 	  // define the values
	let p :f64 = 510000.00;
	let r :f64 = 5.0;
	let n :f64 = 3.0;

	// Depreciation
	let a = p * (1.0 - ( r / 100.0)).powf(n);
	println!("Amount is{}", a);
	let d = p - a;
	println!("Depreciation is{}", d);
}