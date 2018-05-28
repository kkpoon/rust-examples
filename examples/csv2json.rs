extern crate csv;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

type Record = (String, String, Option<u64>);

#[derive(Serialize, Deserialize)]
struct Website {
    name: String,
    url: String,
    rank: Option<u64>,
}

fn example() -> Result<(), Box<Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.deserialize() {
        let record: Record = result?;
        let (name, url, rank) = record;
        let website = Website {
            name: name,
            url: url,
            rank: rank,
        };
        let j = serde_json::to_string(&website)?;
        println!("{}", j);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
