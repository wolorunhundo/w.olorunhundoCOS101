fn main() {
	let t:f64 = 45_0000.0;
	let m:f64 = 15_00000.0;
	let h:f64 = 75_0000.0;
	let d:f64 = 285_0000.0;
	let a:f64 = 25_0000.0;

    //numer of items
	let i = 5.0;
	

	//sum
    let su = t + m + h + d + a;
    println!("Sum is {}",su);

    //average
    let avr = su / i;
    println!("Average is {}", avr);

}