fn main() {

let fullname = "Olorunhundo Omowonuola Darah";
let department = "Data Science";
let uni = "Pan-Atlantic University";


let mut school = "School of Science".to_string();
//push string
school.push_str(" and Technology");

println!("My name is: {}",fullname );

//to know the length 
println!("The length of my full name is: {}",fullname.len() );
println!("I am a student of {} Department",department);
println!("{}",school);
println!("{}",uni);
}
