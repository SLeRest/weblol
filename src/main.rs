mod requestlolapi;
use requestlolapi::RequestLolApi;

fn main() {
    let api_key = "RGAPI-5497e51c-81d7-4acc-8506-393bd4bb9ae9";
    println!("{}", api_key);

    let request =  RequestLolApi::new(api_key, "EUW");
}

/*

   donc le principe c'est de faire une lib en rust qui permet
   d'avoir plein d'informations concernant league of legend

*/
