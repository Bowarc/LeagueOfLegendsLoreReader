#![allow(non_snake_case)]
// #![deny(unused_variables)]

// Note:
//     As we are using asynchronous functions, variables returned by thoses functions are
//     std::future::Future, this means that they are possibly not computed yet, so we have
//     to use `.await` at the end of async functions to get finished vars.
//     See this https://doc.rust-lang.org/std/future/trait.Future.html for more informations
//
// Memo:
//     let json_object: Value = serde_json::Value::String(response_text);
//     let json_object2: Value = serde_json::json!(requestByURL(&url).await);
//     let json_object3: Value = serde_json::from_str(&response_text).expect(&format!("Couldn't create a json object form the response's text."));

use serde_json::Value; //Result
use std::io;
use sublime_fuzzy::{FuzzySearch, Scoring};

mod champion;

const DDRAGON_VERSION: &str = "11.22.1";
const CHAMPION_DATA_URL: &str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion/%CHAMPION%.json";
const LANGUAGE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/languages.json";
const CHAMPION_LIST_URL:&str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion.json";

async fn requestByURL(url: &str) -> String{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", url));

    if r.status() != 200{
        // Problem here, do something
        println!("Problem,\nStatus: {}\nURL: {}", r.status(), url);
    }

    let response_text: String = r.text().await.expect(&format!("Couldn't get the text from the response."));

    response_text
}

fn getClosestMatch(input: String, data: Vec<String>) -> (isize, String){
    let mut best_match: (isize, String) = (0, String::new());

    let scoring = Scoring::emphasize_word_starts();

    for i in data.iter(){
        let result = FuzzySearch::new(&input, i)
            .score_with(&scoring)
            .case_insensitive()
            .best_match();

        match result{
            Some(r) => {
                let score = r.score();
                if score > best_match.0{
                    best_match = (score, i.to_string());
                    println!("New best match: {}, score: {}", i, score);
                }
            },
            None => ()
        }
    }

    best_match
}

async fn getLore(champion: &str, language: &str) -> String{
    if champion == ""{
        return "Could not get the lore of a champ with empty name".to_string()
    }
    let url: String = CHAMPION_DATA_URL
                        .replace("%CHAMPION%", champion)
                        .replace("%LANGUAGE%", language)
                        .replace("%VERSION%", DDRAGON_VERSION);

    let response_text: String = requestByURL(&url).await;

    let json_object: Value = serde_json::from_str(&response_text).expect(&format!("Couldn't create a json object form the response's text."));

    let lore: String = json_object["data"][champion]["lore"].to_string();

    lore
}

fn get_input() -> String{
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input = user_input.trim_end().to_string();

    user_input
}

async fn askLanguage() -> String{
    let languages: String = requestByURL(LANGUAGE_URL).await;
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
    let url = CHAMPION_LIST_URL.replace("%LANGUAGE%", language).replace("%VERSION%", DDRAGON_VERSION);
    
    let response_text = requestByURL(&url).await;

    let champion_list: champion::ChampionList = serde_json::from_str(&response_text).unwrap();
    println!("Please input a champion name:"); 
    let best_match = getClosestMatch(get_input(), champion_list.data.keys().cloned().collect());

       

    best_match.1.to_string()

}


#[tokio::main]
async fn main() {
    println!("Hellow, Welcome.");

    let language = askLanguage().await;
    let champion = askChampion(&language).await;

    let lore = getLore(&champion, &language).await;

    println!("\nLore: {}", lore);
}