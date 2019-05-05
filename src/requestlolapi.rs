extern crate serde_json;
extern crate serde;
extern crate reqwest;

use self::serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Summoner {
    pub id: String,
    pub accountId: String,
    pub puuid: String,
    pub name: String,
    pub profileIconId: u64,
    pub revisionDate: u64,
    pub summonerLevel: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionMastery {
    pub championId: u32,
    pub championLevel: u32,
    pub championPoints: u64,
    pub lastPlayTime: u64,
    pub championPointsSinceLastLevel: u64,
    pub championPointsUntilNextLevel: u64,
    pub chestGranted: bool,
    pub tokensEarned: u64,
}

pub struct RequestLolApi {
    api_key: String,
    region: String,
}

impl RequestLolApi {

    pub fn new(api_key: &str, region: &str) -> Result<Self, String> {
        let uri_region = match region {
            "BR" => "https://br1.api.riotgames.com".to_string(),
            "EUNE" => "https://eun1.api.riotgames.com".to_string(),
            "EUW" => "https://euw1.api.riotgames.com".to_string(),
            "JP" => "https://jp1.api.riotgames.com".to_string(),
            "KR" => "https://kr.api.riotgames.com".to_string(),
            "LAN" => "https://la1.api.riotgames.com".to_string(),
            "LAS" => "https://la2.api.riotgames.com".to_string(),
            "NA" => "https://na1.api.riotgames.com".to_string(),
            "OCE" => "https://oc1.api.riotgames.com".to_string(),
            "TR" => "https://tr1.api.riotgames.com".to_string(),
            "RU" => "https://ru.api.riotgames.com".to_string(),
            "PBE" => "https://pbe1.api.riotgames.com".to_string(),
            _ => {
                return Err(format!("{}{}", "Wrong Region: ", region))
            },
        };
        Ok(RequestLolApi {
            api_key: api_key.to_string(),
            region: uri_region,
        })

    }

    pub fn summoner(&self, summoner_name: &str)
        -> Result<Summoner, Box<std::error::Error>>
    {
        let path = "/lol/summoner/v4/summoners/by-name/";
        let url = format!("{}{}{}?api_key={}",
                    self.region, path, summoner_name, self.api_key);
        let Summoner: Summoner = reqwest::get(&url)?.json()?;
        Ok(Summoner)
   }

    pub fn champion_masteries(&self, encrypted_summoner_id: &str)
        -> Result<Vec<ChampionMastery>, Box<std::error::Error>>
    {
        let path = "/lol/champion-mastery/v4/champion-masteries/by-summoner/";
        let url = format!("{}{}{}?api_key={}",
                    self.region, path, encrypted_summoner_id, self.api_key);
        let champ_master: Vec<ChampionMastery> = reqwest::get(&url)?.json()?;
        Ok(champ_master)
    }
}
