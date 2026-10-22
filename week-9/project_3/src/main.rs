use std::io::Write;

fn main() {
    let commissioner = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministry = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"];

    let zone = vec![
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",];

    println!("{:<30} | {:<20} | {:<15}", "Commissioner", "Ministry", "Zone");
    println!("{}", "-".repeat(70));

    for i in 0..commissioner.len() {
        println!("{:<30} | {:<20} | {:<15}", commissioner[i], ministry[i], zone[i]);
    }
    let mut file = std::fs::File::create("EFCC Merge.csv").expect("create failed"); 
    let mut file_content = String::from("Commissioner,Ministry,Zone\n");

    for i in 0..commissioner.len() {
        let row = format!("{},{},{}\n", commissioner[i], ministry[i], zone[i]);
        file_content.push_str(&row);
    }

    file.write_all(file_content.as_bytes()).expect("write failed");

    println!("\nSuccessfully merged datasets and saved to EFCC Merge.csv.");
}
  
    
