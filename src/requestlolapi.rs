extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use self::futures::{future, Future};
use self::hyper_tls::HttpsConnector;
use self::hyper::Client;

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
        -> String
        {
            let response = String::new();
            tokio::run(future::lazy(|| {
                let https = HttpsConnector::new(1).unwrap();
                let client = Client::builder()
                    .build::<_, hyper::Body>(https);

                client
                    .get("https://hyper.rs".parse().unwrap())
                    .map(|res| {
                        if res.status() == 200 {
                            response = res.body().Payload();
                        }
                    })
                .map_err(|e| println!("request error: {}", e))
            }));
            response
        }
}
