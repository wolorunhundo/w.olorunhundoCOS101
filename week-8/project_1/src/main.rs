use std::io;

fn main() {
    // vector of tuples: (min_years, max_years, level_name)
    let levels = vec![
        (1, 2, "APS 1-2"),
        (3, 5, "APS 3-5"),
        (5, 8, "APS 5-8"),
        (8, 10, "EL1 8-10"),
        (10, 13, "EL2 10-13"),
        (13, 30, "SES"),
    ];

    let mut input_txt = String::new();
    println!("Enter years of experience:");
    io::stdin().read_line(&mut input_txt).expect("failed to read");
    
    // basic parse just like practice 2 & 5
    let yrs: u32 = input_txt.trim().parse().expect("please enter a valid number");

    let mut matched = false;

    // loop through the vector to check ranges
    for item in levels {
        if yrs >= item.0 && yrs <= item.1 {
            println!("Level found: {}", item.2);
            matched = true;
            break;
        }
    }

    if !matched {
        println!("Experience is out of range or invalid.");
    }
}