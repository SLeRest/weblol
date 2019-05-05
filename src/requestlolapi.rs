extern crate serde_json;
extern crate serde;
extern crate reqwest;

use self::serde::{Serialize, Deserialize};
use self::serde_json::{Value, json};
use std::fmt;

pub struct RequestLolApi {
    api_key: String,
    region: String,
}

impl RequestLolApi {

    pub fn new(metadata: Value) -> Result<Self, String> {
        let uri_region = match metadata["region"] {
            json!("BR") => "https://br1.api.riotgames.com".to_string(),
            json!("EUNE") => "https://eun1.api.riotgames.com".to_string(),
            json!("EUW") => "https://euw1.api.riotgames.com".to_string(),
            json!("JP") => "https://jp1.api.riotgames.com".to_string(),
            json!("KR") => "https://kr.api.riotgames.com".to_string(),
            json!("LAN") => "https://la1.api.riotgames.com".to_string(),
            json!("LAS") => "https://la2.api.riotgames.com".to_string(),
            json!("NA") => "https://na1.api.riotgames.com".to_string(),
            json!("OCE") => "https://oc1.api.riotgames.com".to_string(),
            json!("TR") => "https://tr1.api.riotgames.com".to_string(),
            json!("RU") => "https://ru.api.riotgames.com".to_string(),
            json!("PBE") => "https://pbe1.api.riotgames.com".to_string(),
            _ => {
                return Err(format!("{}{}", "Wrong Region: ", metadata["region"]))
            },
        };
        Ok(RequestLolApi {
            api_key: metadata["api_key"].to_string(),
            region: uri_region,
        })

    }

    pub fn summoner(&self, summoner_name: &str)
        -> Result<Value, Box<std::error::Error>>
    {
        let path = "/lol/summoner/v4/summoners/by-name/";
        let url = format!("{}{}{}?api_key={}",
                    self.region, path, summoner_name, self.api_key);
        let summoner: Value = reqwest::get(&url)?.json()?;
        Ok(summoner)
   }

    pub fn champion_masteries(&self, encrypted_summoner_id: &str)
        -> Result<Vec<Value>, Box<std::error::Error>>
    {
        let path = "/lol/champion-mastery/v4/champion-masteries/by-summoner/";
        let url = format!("{}{}{}?api_key={}",
                    self.region, path, encrypted_summoner_id, self.api_key);
        let champ_master: Vec<Value> = reqwest::get(&url)?.json()?;
        Ok(champ_master)
    }
}
