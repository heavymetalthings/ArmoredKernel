use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn read_data_from_file(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}


fn main() {
    let path = "src/examplefile.arp";
    match read_data_from_file(path) {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("Error: {:?}", err),
    }
}