use csv::Reader;
use std::error::Error;

fn read_csv() -> Result<(), Box<dyn Error>> {
    let file_path = "" ; // add the path to your file here
    let mut reader = Reader::from_path(file_path)?;
    let headers = reader.headers()?.clone();
    let data: Vec<Vec<String>> = reader.records().map(|record| {
        record.unwrap().iter().map(|field| field.to_string()).collect()
    }).collect();
    
    println!("headers: {:?}", headers);
    println!("data: {:?}", data);

    Ok(())
}




fn main() {
    if let Err(e) = read_csv() {
        println!("Error: {}", e);
    }
}