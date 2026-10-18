fn main() {
    // vector of tuples: (name, years) based on the hint in the slides
    let applicants = vec![
        ("Tolu", 2),
        ("Blessing", 5),
        ("Emmanuel", 4),
        ("Chidi", 7),
        ("Amina", 3),
    ];

    let mut best_person = applicants[0].0;
    let mut max_yrs = applicants[0].1;

    // looping through using indices like practice 4 and 6
    for i in 0..applicants.len() {
        let current_name = applicants[i].0;
        let current_yrs = applicants[i].1;

        if current_yrs > max_yrs {
            max_yrs = current_yrs;
            best_person = current_name;
        }
    }

    println!("The best candidate for EY Nigeria is {} with {} years of experience!", best_person, max_yrs);
}