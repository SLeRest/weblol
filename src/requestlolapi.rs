extern crate hyper;

use urilolapi::UriLolAPI;
use self::hyper::{Client, Uri};

pub struct RequestLolApi {
    uri: UriLolAPI, 
}

impl RequestLolApi {
    pub fn summoner(&self, summoner_name: &str)
        -> Client::ResponseFuture
        {
            let client = Client::new();
            let url = self.uri.summoner(summoner_name).unwrap();
            let mut response = client.get(url);

            return response;
        }
}
