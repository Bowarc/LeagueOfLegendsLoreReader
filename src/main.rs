#![allow(non_snake_case)]
#![warn(unused_variables)]
use std::collections::HashMap;

const URL : &str = "https://ddragon.leagueoflegends.com/cdn/10.3.1/data/en_EN/champion/LeeSin.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::builder()
        .build()?;

    let res = client
        .get(URL)
        .send()
        .await?;

    // ?  
    let ip = res
        .json::<HashMap<String, String>>()
        .await?;
    

    println!("{:?}", ip);

    Ok(())
}
