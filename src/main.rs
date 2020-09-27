#[macro_use]
extern crate clap;

use std::error::Error;
use csv::Reader;
use std::collections::HashMap;

type Record = HashMap<String, String>;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(hotzipcodes =>
        (version: "1.0")
        (author: "Tracy B. <tracy@brown-rei.com>")
        (about: "Takes a csv file with cash transactions and lists the hot zip codes")
        (@arg INPUT: +required "CSV file with cash transactions")
        (@arg HEADER: -z --header +takes_value "Sets a header to look for the zip code in (default: zip)")
    ).get_matches();

    let cash_csv = matches.value_of("INPUT").unwrap();
    let header = matches.value_of("HEADER").unwrap_or("zip");
    println!("Hot zipcodes from {}:", cash_csv);

    let mut zipcodes = HashMap::new();
    let mut reader = Reader::from_path(cash_csv)?;
    for result in reader.deserialize() {
        let record: Record = result?;
        let key = record[header].clone();
        *zipcodes.entry(key).or_insert(0) += 1;
    }

    let mut sorted: Vec<(&String, &i32)> = zipcodes.iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(b.1).reverse());

    for (zip, count) in sorted {
        println!("{}: {}", zip, count);
    }

    Ok(())
}
