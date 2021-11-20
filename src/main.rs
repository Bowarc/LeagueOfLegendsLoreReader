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
use tts;

mod champion;

const DDRAGON_VERSION_URL: &str = "https://ddragon.leagueoflegends.com/api/versions.json";
const CHAMPION_DATA_URL: &str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion/%CHAMPION%.json";
const LANGUAGE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/languages.json";
const CHAMPION_LIST_URL:&str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion.json";

struct LoreReader{
    ddragon_latest_version: String,
    champion_list: champion::ChampionList,
    language_list: Vec<String>,
    selected_champion: String,
    selected_language: String,
}

impl LoreReader{
    fn new() -> Self{
        Self{
            ddragon_latest_version: String::new(),
            champion_list: Default::default(),
            language_list: Vec::new(),
            selected_champion: String::new(),
            selected_language: String::new(),
        }
    }

    async fn init(&mut self){
        self.set_ddragon_version().await;

        self.get_language_list().await;
        
        self.ask_language();

        self.get_champion_list().await;
    }

    async fn run(&mut self){
        self.init().await;
        println!("");
        self.ask_chamption();
        println!("");

        let lore = self.getLore().await;

        println!("Lore: {}", lore);
        speak(lore);
        
        // self.get_champion().await;
    }

    async fn set_ddragon_version(&mut self){
        let version_data: String = requestByURL(DDRAGON_VERSION_URL).await;
        let version_list: Vec<String> = serde_json::from_str(&version_data).expect(&format!("Couldn't transfrom version data to vec of string."));

        self.ddragon_latest_version = version_list[0].clone();

        println!("version: {}", self.ddragon_latest_version);
    }

    async fn get_language_list(&mut self) {
        let languages_data: String = requestByURL(LANGUAGE_URL).await;
        let language_list: Vec<String> = serde_json::from_str(&languages_data).expect(&format!("Couldn't create a json object form the response's text."));
        
        self.language_list = language_list.clone();
    }

    fn ask_language(&mut self){
        if self.language_list.is_empty(){
            panic!("Yikes, the language_list is empty");
        }
        let mut selected_language = String::new();

        while selected_language == String::new(){
            for lang in self.language_list.chunks(5){
                let s = format!("{:02?}", lang).replace('"', "").replace("[", "").replace("]", "").replace(" ", "  ");

                println!("{}",s);
            }
            println!("Please choose a language from the list above ^:");

            let user_input = get_input();

            let closest_match = getClosestMatch(user_input, self.language_list.clone());
            if closest_match == String::new(){
                println!("Failes to recognise the selected_language.");
            }else{
                selected_language = closest_match;
            }
        }
        self.selected_language = selected_language;
    }

    async fn get_champion_list(&mut self){
        let url = CHAMPION_LIST_URL.replace("%LANGUAGE%", &self.selected_language).replace("%VERSION%", &self.ddragon_latest_version);

        let champion_data: String = requestByURL(&url).await;

        let champion_list: champion::ChampionList = serde_json::from_str(&champion_data).unwrap();
        
        self.champion_list = champion_list;
    }

    fn ask_chamption(&mut self){
        let mut selected_champion = String::new();

        while selected_champion == String::new(){
            println!("Please input a champion name:"); 
            let user_input = get_input();

            let closest_match = getClosestMatch(user_input, self.champion_list.data.keys().cloned().collect());

            if closest_match == String::new(){
                println!("Failed to recognise the selected champion.");
            }else{
                selected_champion = closest_match;
            }
        }  
        self.selected_champion = selected_champion;
    }

    async fn getLore(&self) -> String{
        if self.selected_champion == String::new(){
            return "Could not get the lore of a champ with empty name".to_string()
        }
        let url: String = CHAMPION_DATA_URL
                            .replace("%CHAMPION%", &self.selected_champion)
                            .replace("%LANGUAGE%", &self.selected_language)
                            .replace("%VERSION%", &self.ddragon_latest_version);

        let response_text: String = requestByURL(&url).await;

        let json_object: Value = serde_json::from_str(&response_text).expect(&format!("Couldn't create a json object form the response's text."));

        let lore: String = json_object["data"][self.selected_champion.clone()]["lore"].to_string();

        lore
    }
}

fn speak(text: String){
    let mut speech = tts::Tts::new(tts::Backends::WinRt).unwrap();
    speech.set_volume(0.03).unwrap();
    speech.speak(text, false).unwrap();

    // println!("Volume: {:?}", speech.get_volume());
    let _ = get_input();
}

async fn requestByURL(url: &str) -> String{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", url));

    if r.status() != 200{
        // Problem here, do something
        println!("Problem,\nStatus: {}\nURL: {}", r.status(), url);
    }

    let response_text: String = r.text().await.expect(&format!("Couldn't get the text from the response."));

    response_text
}

fn getClosestMatch(input: String, data: Vec<String>) -> String{
    fn newBestMatch(score: isize, name: &str) -> (isize, String) {
        (score, name.to_string())
    }

    let mut best_match: (isize, String) = (0, String::new());

    for i in data.iter(){
        let result = FuzzySearch::new(&input, &i.to_lowercase())
            .score_with(&Scoring::emphasize_word_starts())
            .best_match();

        match result{
            Some(r) => {
                let score = r.score();
                if score > best_match.0{
                    best_match = newBestMatch(score, i)
                }
            },
            None => ()
        }
        if input == i.to_lowercase(){
            best_match = newBestMatch(-1, i);

            break
        }
    }
    println!("Best match: {}", best_match.1);

    best_match.1
}

fn get_input() -> String{
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input = user_input.trim_end().to_string();

    user_input
}

#[tokio::main]
async fn main() {
    println!("Hellow, Welcome.\n");

    let mut lore_reader = LoreReader::new();

    lore_reader.run().await;
}