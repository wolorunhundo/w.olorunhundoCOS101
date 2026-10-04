use std::io;

fn main() {
    // Display the restaurant menu to the customer
    println!("========================================");
    println!("            RESTAURANT MENU             ");
    println!("========================================");
    println!("P -> Poundo Yam / Edinkaiko Soup (N3,200)");
    println!("F -> Fried Rice & Chicken        (N3,000)");
    println!("A -> Amala & Ewedu Soup          (N2,500)");
    println!("E -> Eba & Egusi Soup            (N2,000)");
    println!("W -> White Rice & Stew           (N2,500)");
    println!("========================================");

    // Read the food type choice from the customer
    println!("\nEnter the letter of your food choice (P, F, A, E, W):");
    let mut food_choice = String::new();
    io::stdin()
        .read_line(&mut food_choice)
        .expect("Failed to read input");
    let food_choice = food_choice.trim().to_uppercase();

    // Read the quantity from the customer
    println!("Enter the quantity:");
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");
    
    // Parse quantity into an integer, handle any input errors gracefully
    let quantity: u32 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered. Please restart and enter a valid number.");
            return;
        }
    };

    // Determine the price per unit using a match expression based on the letter
    let price_per_unit = match food_choice.as_str() {
        "P" => 3200.0, // Poundo Yam / Edinkaiko Soup
        "F" => 3000.0, // Fried Rice & Chicken
        "A" => 2500.0, // Amala & Ewedu Soup
        "E" => 2000.0, // Eba & Egusi Soup
        "W" => 2500.0, // White Rice & Stew
        _ => {
            println!("Invalid menu selection! Please choose a valid letter (P, F, A, E, W).");
            return;
        }
    };

    // Compute the total charge (arithmetic for the total)
    let mut total_charge = price_per_unit * (quantity as f64);

    // Check if the total is greater than N10,000 to apply the 5% discount
    if total_charge > 10000.0 {
        let discount = total_charge * 0.05;
        total_charge -= discount;
        println!("\nCongratulations! Your order exceeds N10,000, so a 5% discount of N{:.2} has been applied.", discount);
    }

    // Display the final total charge to the customer
    println!("Your final total charge is: N{:.2}", total_charge);
}