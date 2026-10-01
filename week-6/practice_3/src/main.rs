fn main() {
    let name1 = "Olorunhundo Omowonuola";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Omowonuola","Darah");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty","Institution");
    println!("I am a student of the {}",school );
}
