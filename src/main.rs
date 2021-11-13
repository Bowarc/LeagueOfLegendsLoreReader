#![allow(non_snake_case)]
#![deny(unused_variables)]

use serde_json::Value; //Result

const DDRAGON_URL : &str = "https://ddragon.leagueoflegends.com/cdn/10.3.1/data/%LANGUAGE%/champion/%CHAMPION%.json";

async fn getURL(url: String) -> Value{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", DDRAGON_URL));

    let response_text: String = r.text().await.expect(&format!("Couldn't get the text from the response."));
    
    let j: Value = serde_json::from_str(&response_text).expect(&format!("Couldn't create a json object form the response's text."));
    
    j
}

#[tokio::main]
async fn main() {
    let champion: &str = "LeeSin";
    let language: &str = "fr_FR";

    let url: String = DDRAGON_URL.replace("%CHAMPION%", champion)
                        .replace("%LANGUAGE%", language);

    let LeeJsonObject: Value = getURL(url).await;

    println!("{}", LeeJsonObject["data"]["LeeSin"]["lore"].to_string());
}