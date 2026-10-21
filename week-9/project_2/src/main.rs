use std::io::Write;

fn main() {
    let students = vec![
    ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
    ("Adams Aliyu", "ECO10110101", "Economics", "100"),
    ("Shania Bolade", "CSC10328828", "Computer", "200"),
    ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
    ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];
    println!("{:<20}| {:<15}| {:<12}| {:<5}","Student Name", "Matric. Number", "Department", "Level" );
    println!("{}","-".repeat(60));

    for student in &students {
       println!("{:<20}| {:<15}| {:<12}| {:<5}", student.0, student.1, student.2, student.3); 
    }

    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");

    let mut file_content = String::from("Student Name,Matric. Number,Department,Level\n");

    for student in &students {
        let row = format!("{},{},{},{}\n", student.0, student.1, student.2, student.3);
        file_content.push_str(&row);
    }

    file.write_all(file_content.as_bytes()).expect("write failed");

    println!("\nStudent records successfully written to PAU-SMIS.csv.");
}


