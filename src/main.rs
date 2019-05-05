mod requestlolapi;
use requestlolapi::RequestLolApi;

fn main() {
    let api_key = "RGAPI-5497e51c-81d7-4acc-8506-393bd4bb9ae9";

    let request =  RequestLolApi::new(api_key, "EUW").unwrap();
    // error handler
    let summoner = request.summoner("Ouralgan").unwrap();
}
