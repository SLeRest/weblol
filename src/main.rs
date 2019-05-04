mod urilolapi;
pub use urilolapi::UriLolAPI;

fn main() {
    let api_key = "RGAPI-417ae5ac-eaec-4081-8d70-6bbdc374b0b5";
    let uri_lol_api = match UriLolAPI::new(api_key, "EUW") {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::exit(1);
        },
    };

    let summoner_name = "Ouralgan";
    let uri_summoner = uri_lol_api.summoner(summoner_name).unwrap();
    println!("{}", uri_summoner);
    let test = uri_lol_api.total_masteries_score("OUIOUI").unwrap();
    println!("{}", test);

    // donc maintenant je veux requetter ces url
    // il me faut une struct 
}

/*

   donc le principe c'est de faire une lib en rust qui permet
   d'avoir plein d'informations concernant league of legend

*/
