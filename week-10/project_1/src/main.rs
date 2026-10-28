/*My code to calculate the total cost of 3 quantities of 
each laptop from Alaba electronics*/

struct AlabaElectronics{
    brand:String,
    price:u64,
}
impl AlabaElectronics{
    fn calculate_cost(&self,quantity:u32)->u64{
        self.price * (quantity as u64)
    }
}

fn main() {
    let hp = AlabaElectronics{
        brand:String::from("HP"),
        price: 650_000,
    };
    let ibm = AlabaElectronics{
        brand:String::from("IBM"),
        price:755_000,
    };
    let toshiba = AlabaElectronics{
        brand:String::from("Toshiba"),
        price:550_000,
    };
    let dell = AlabaElectronics{
        brand:String::from("Dell"),
        price: 850_000,
    };

let quantity_of_computers_to_buy = 3;
let hp_cost = hp.calculate_cost(quantity_of_computers_to_buy);
let ibm_cost = ibm.calculate_cost(quantity_of_computers_to_buy);
let toshiba_cost = toshiba.calculate_cost(quantity_of_computers_to_buy);
let dell_cost = dell.calculate_cost(quantity_of_computers_to_buy);

println!("The cost for {} {} laptops are ={}",quantity_of_computers_to_buy,hp.brand,hp_cost );
println!("The cost for {} {} laptops are ={}",quantity_of_computers_to_buy,ibm.brand,ibm_cost );
println!("The cost for {} {} laptops are ={}",quantity_of_computers_to_buy,toshiba.brand,toshiba_cost );
println!("The cost for {} {} laptops are ={}",quantity_of_computers_to_buy,dell.brand,dell_cost );

let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

println!("---------------------------------------------------------------" );
println!("THE TOTAL COST FOR 3 LAPTOPS FROM EACH BRAND IS -> {}",total_cost );
println!("----------------------------------------------------------------");
}