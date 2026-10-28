fn add_one(a: &mut i32) {
        *a+=1;

    }
fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}",i );
}
