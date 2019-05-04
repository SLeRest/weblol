extern crate http;
use self::http::{Uri, request, Response};

pub struct UriLolAPI {
    api_key: String,
    region: String,
}

impl UriLolAPI {
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
        Ok(UriLolAPI {
            api_key: api_key.to_string(),
            region: uri_region,
        })
    }

    pub fn summoner(&self, summoner_name: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
            let url_summoner = "/lol/summoner/v4/summoners/by-name/";
            let url = format!("{}{}{}?api_key={}",
                                     self.region, url_summoner,
                                     summoner_name, self.api_key);
            url.parse::<Uri>()
    }

    pub fn summoner_by_id(&self, summoner_id: &str)
       -> Result<http::Uri, http::uri::InvalidUri>
    {
            let url_summoner_by_id = "/lol/summoner/v4/summoners/";
            let url = format!("{}{}{}?api_key={}",
                                     self.region, url_summoner_by_id,
                                     summoner_id, self.api_key);
            url.parse::<Uri>()
    }

    pub fn all_champions_masteries(&self, encrypted_summoner_id: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_all_champ_mast =  "/lol/champion-mastery/v4/\
                                   champion-masteries/by-summoner/";
        let url = format!("{}{}{}?api_key={}",
                                self.region, url_all_champ_mast,
                                encrypted_summoner_id, self.api_key);
        url.parse::<Uri>()
    }

    pub fn champion_masteries(&self, encrypted_summoner_id: &str, champion_id: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_champ_mast =  "/lol/champion-mastery/v4/\
                               champion-masteries/by-summoner/";
        let url = format!("{}{}{}/by-champion/{}?api_key={}",
                                self.region, url_champ_mast,
                                encrypted_summoner_id, champion_id,
                                self.api_key);
        url.parse::<Uri>()

    }

    pub fn total_masteries_score(&self, encrypted_summoner_id: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_total_mast_score =  "/lol/champion-mastery/v4/\
                                     scores/by-summoner/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_total_mast_score,
                                encrypted_summoner_id, self.api_key);
        url.parse::<Uri>()

    }

   pub fn champion_rotations(&self)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_champ_rot =  "/lol/platform/v3/champion-rotations";
        let url = format!("{}{}/?api_key={}",
                                self.region, url_champ_rot, self.api_key);
        url.parse::<Uri>()

    }

    pub fn challenger_league(&self, queue: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_chall_league =  "/lol/league/v4/challengerleagues/by-queue/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_chall_league,
                                queue, self.api_key);
        url.parse::<Uri>()

    }

    pub fn league_entries_summoner(&self, encrypted_summoner_id: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_league_entries_sum = "/lol/league/v4/entries/by-summoner/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_league_entries_sum,
                                encrypted_summoner_id, self.api_key);
        url.parse::<Uri>()
    }

    pub fn all_league_entries(&self, queue: &str, division: &str, tier: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_all_league_entries = "/lol/league/v4/entries/";
        let url = format!("{}{}{}/{}/{}?api_key={}",
                                self.region, url_all_league_entries,
                                queue, tier, division, self.api_key);
        url.parse::<Uri>()
    }

    pub fn grandmaster_league(&self, queue: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_grandmaster_league =  "/lol/league/v4/\
                                       grandmasterleagues/by-queue/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_grandmaster_league,
                                queue, self.api_key);
        url.parse::<Uri>()
    }
    
    pub fn league(&self, league_id: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_league = "/lol/league/v4/leagues/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_league,
                                league_id, self.api_key);
        url.parse::<Uri>()

    }

    pub fn master_league(&self, queue: &str)
        -> Result<http::Uri, http::uri::InvalidUri>
    {
        let url_master_league =  "/lol/league/v4/masterleagues/by-queue/";
        let url = format!("{}{}{}/?api_key={}",
                                self.region, url_master_league,
                                queue, self.api_key);
        url.parse::<Uri>()
    }

}
