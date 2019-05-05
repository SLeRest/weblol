extern crate serde_json;
mod requestlolapi;

use requestlolapi::RequestLolApi;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use self::serde_json::Value;

fn metadata() -> std::io::Result<serde_json::Value> {
    let file = File::open("/home/ouralgan/lol_web/lolapi/data/lolapi.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let metadata: Value = serde_json::from_str(&contents)?;
    Ok(metadata)
}

fn main() {
    let metadata = metadata().unwrap();

    let request =  RequestLolApi::new(metadata).unwrap();
    let summoner = request.summoner("Ouralgan").unwrap();
}
