extern crate serde;
extern crate serde_json;
extern crate reqwest;

use self::serde::{Serialize, Deserialize};
use self::serde_json::{Value, json};
use std::fmt;

pub struct RequestLolApi {
    api_key: String,
    region: String,
}

impl RequestLolApi {

    pub fn new(metadata: &Value) -> Result<Self, String> {
        let uri_region = match metadata["region"].as_str() {
            Some("BR") => "https://br1.api.riotgames.com".to_string(),
            Some("EUNE") => "https://eun1.api.riotgames.com".to_string(),
            Some("EUW") => "https://euw1.api.riotgames.com".to_string(),
            Some("JP") => "https://jp1.api.riotgames.com".to_string(),
            Some("KR") => "https://kr.api.riotgames.com".to_string(),
            Some("LAN") => "https://la1.api.riotgames.com".to_string(),
            Some("LAS") => "https://la2.api.riotgames.com".to_string(),
            Some("NA") => "https://na1.api.riotgames.com".to_string(),
            Some("OCE") => "https://oc1.api.riotgames.com".to_string(),
            Some("TR") => "https://tr1.api.riotgames.com".to_string(),
            Some("RU") => "https://ru.api.riotgames.com".to_string(),
            Some("PBE") => "https://pbe1.api.riotgames.com".to_string(),
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
