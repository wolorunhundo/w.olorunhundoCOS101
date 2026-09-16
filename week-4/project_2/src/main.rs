// Programme to Determine the annual incentive of employees
use std::io;
fn main() {

    println!("Are you experienced? (YES/NO):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    println!("Enter your age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: i32 = age_input.trim().parse().expect("Not a valid number");

    let incentive = if is_experienced {
        if age >= 40 {
            1_560_000.00
        } else if age >= 30 && age <= 39 {
            1_480_000.00
        } else if age < 28 {
            1_300_000.00
        } else {
            100_000.00 // Default fallback for age 28–29
        }
    } else {
        100_000.00
    };

    println!("Annual incentive: N{}", incentive);
    
    
}
