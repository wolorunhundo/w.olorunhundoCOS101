use std::io;

// Helper function so we don't have to keep writing boilerplate input code 
// (total lifesaver for handling inputs)
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line, bruh");
    
    // Clean up user input and parse it, default to 0.0 if they type random letters
    input.trim().parse().unwrap_or_else(|_| {
        println!("That's not a number smh. Using 0.0 instead.");
        0.0
    })
}

fn calculate_trapezium() {
    println!("\n--- Trapezium Area ---");
    let height = get_input("Enter height:");
    let base1 = get_input("Enter base 1:");
    let base2 = get_input("Enter base 2:");
    
    // Formula: height / 2 * (base1 + base2)[cite: 1]
    let area = (height / 2.0) * (base1 + base2);
    println!("--> Result: Area = {}\n", area);
}

fn calculate_rhombus() {
    println!("\n--- Rhombus Area ---");
    let d1 = get_input("Enter diagonal 1:");
    let d2 = get_input("Enter diagonal 2:");
    
    // Formula: 1/2 * diagonal1 * diagonal2[cite: 1]
    let area = 0.5 * d1 * d2;
    println!("--> Result: Area = {}\n", area);
}

fn calculate_parallelogram() {
    println!("\n--- Parallelogram Area ---");
    let base = get_input("Enter base:");
    let altitude = get_input("Enter altitude:");
    
    // Formula: base * altitude[cite: 1]
    let area = base * altitude;
    println!("--> Result: Area = {}\n", area);
}

fn calculate_cube() {
    println!("\n--- Cube Surface Area ---");
    let side = get_input("Enter side length:");
    
    // Formula: 6 * side * side[cite: 1]
    let surface_area = 6.0 * side * side;
    println!("--> Result: Surface Area = {}\n", surface_area);
}

fn calculate_cylinder() {
    println!("\n--- Cylinder Volume ---");
    let radius = get_input("Enter radius:");
    let height = get_input("Enter height:");
    
    // Formula: pi * radius * radius * height[cite: 1]
    const PI: f64 = 3.1415926535;
    let volume = PI * radius * radius * height;
    println!("--> Result: Volume = {}\n", volume);
}

fn main() {
    // Infinite loop so we don't have to keep restarting the program every time we test a shape
    loop {
        println!("===============================");
        println!("  SHAPE CALCULATOR (MTH 101)   ");
        println!("===============================");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        
        let choice = get_input("Pick a choice (1-6):");
        let choice_int = choice as i32;

        if choice_int == 1 {
            calculate_trapezium();
        } else if choice_int == 2 {
            calculate_rhombus();
        } else if choice_int == 3 {
            calculate_parallelogram();
        } else if choice_int == 4 {
            calculate_cube();
        } else if choice_int == 5 {
            calculate_cylinder();
        } else if choice_int == 6 {
            println!("Peace out, hope the lecturer gives us an A ✌️");
            break;
        } else {
            println!("Bro, that's not even on the menu. Try again.\n");
        }
    }
}