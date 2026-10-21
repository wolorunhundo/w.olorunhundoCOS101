use std::io::Write;

fn main(){


    let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("create failed");

    let header = ("Lager","Stout","Non-Alcoholic");
    let row1 = ("33 Export","Legend","Maltina");
    let row2 = ("Desperados","Turbo King","Amstel Malta");
    let row3 = ("Goldberg","Williams","Malta Gold");
    let row4 = ("Gulder","","Fayrouz");
    let row5 = ("Heineken","","");
    let row6 = ("Star","", "");
    
    let content = format!(
       "{},{},{}\n
       {},{},{}\n
       {},{},{}\n
       {},{},{}\n
       {},{},{}\n
       {},{},{}\n
       {},{},{}\n", header.0, header.1, header.2,
                    row1.0, row1.1, row1.2,
                    row2.0, row2.1, row2.2,
                    row3.0, row3.1, row3.2,
                    row4.0, row4.1, row4.2,
                    row5.0, row5.1, row5.2,
                    row6.0, row6.1, row6.2

        );
    file.write_all(content.as_bytes()).expect("write failed");
    println!("Data has been written to NIGERIAN BREWERIES" );


    
}
