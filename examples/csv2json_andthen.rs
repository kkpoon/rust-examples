extern crate csv;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

#[derive(Serialize, Deserialize)]
struct Website {
    name: String,
    url: String,
    rank: Option<u64>,
}

fn example() -> Result<(), Box<Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.deserialize() {
        match result
            .map(|(name, url, rank)| Website {
                name: name,
                url: url,
                rank: rank,
            })
            .map_err(|_e: csv::Error| "parse csv error")
            .and_then(|website| {
                serde_json::to_string(&website).map_err(|_e: serde_json::Error| "parse json error")
            }) {
            Ok(j) => println!("{}", j),
            Err(err) => println!("{}", err),
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
