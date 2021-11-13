#![allow(non_snake_case)]
// #![deny(unused_variables)]

// Note:
//     As we are using asynchronous functions, variables returned by thoses functions are
//     std::future::Future, this means that they are possibly not computed yet, so we have
//     to use `.await` at the end of async functions to get finished vars.
//     See this https://doc.rust-lang.org/std/future/trait.Future.html for more informations

use serde_json::Value; //Result
use std::io;


const CHAMPION_DATA_URL : &str = "https://ddragon.leagueoflegends.com/cdn/10.3.1/data/%LANGUAGE%/champion/%CHAMPION%.json";
const LANGUAGE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/languages.json";
const CHAMPION_LIST_URL:&str = "https://ddragon.leagueoflegends.com/cdn/10.3.1/data/%LANGUAGE%/champion.json";

async fn getURL(url: &str) -> String{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", url));

    if r.status() != 200{

        // Problem here, do something
    }
    println!("Status: {}", r.status());

    let response_text: String = r.text().await.expect(&format!("Couldn't get the text from the response."));

    response_text
}
// 

async fn getLore(champion: &str, language: &str) -> String{
    let url: String = CHAMPION_DATA_URL.replace("%CHAMPION%", champion)
                        .replace("%LANGUAGE%", language);

    let response_text: String = getURL(&url).await;

    let json_object: Value = serde_json::from_str(&response_text).expect(&format!("Couldn't create a json object form the response's text."));

    let lore: String = json_object["data"][champion]["lore"].to_string();

    lore
}

fn get_input() -> String{
    let mut user_input = String::new();

    // listening for user input
    io::stdin().read_line(&mut user_input).unwrap();

    // make the user input a String
    user_input = user_input.trim_end().to_string();
    user_input
}

fn get_type<T>(_: &T)-> std::any::TypeId{
    std::any::TypeId::of::<String>()
}

async fn askLanguage() -> String{
    let languages: String = getURL(LANGUAGE_URL).await;
    let language_list: Vec<&str> = serde_json::from_str(&languages).expect(&format!("Couldn't create a json object form the response's text."));
    
    let answer: String;
    loop{
        println!("Please choose a language in this list:");
        for lang in language_list.chunks(5){
            let s = format!("{:02?}", lang);
            let s = s.replace('"', "").replace("[", "").replace("]", "").replace(" ", "  ");
            println!("{}", s);
        }
        let user_input: &str = &get_input();

        if language_list.contains(&user_input){
            answer = user_input.to_string();
            break
        }else{
            println!("Invalid choise: {}\n", user_input);
        }
    }
    answer
}

async fn askChampion(language: &str) -> String{
    let url = CHAMPION_LIST_URL.replace("%LANGUAGE%", language);
    let response_text = getURL(&url).await;

    "LeeSin".to_string()
}

#[tokio::main]
async fn main() {
    println!("Hellow, Welcome.");

    let language = askLanguage().await;
    let champion = askChampion(&language).await;

    let lore = getLore(&"LeeSin", &"en_US").await;
 
}