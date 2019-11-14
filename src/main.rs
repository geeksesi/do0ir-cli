extern crate clipboard;
extern crate reqwest;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Do0result {
    success: bool,
    #[serde(deserialize_with = "parse_null")]
    error: String,
    #[serde(deserialize_with = "parse_null")]
    address: String,
    #[serde(deserialize_with = "parse_null")]
    short: String,
}

fn parse_null<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("null".to_string()))
}

fn get_request(link: String) -> Result<Do0result, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [("link", link)];
    let response: Do0result = client
        .post("https://do0.ir/post/sD9qHKZaUC7g/2.5/code")
        .form(&params)
        .send()?
        .json()?;

    Ok(response)
}

fn check_is_url(url: String) -> String {
    if (url.find("http://") != None) || (url.find("https://") != None) {
        return url;
    } else {
        return "".to_string();
    }
}

fn copy_to_clipboard(shorted: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(shorted).unwrap();
}

fn main() {
    let _args = env::args();
    let mut i = 0;
    let mut link: String = "".to_string();
    for argument in _args {
        if i == 1 {
            link = argument;
        }
        i += 1;
    }

    let link: String = check_is_url(link);
    if link != "" {
        let do0_answer: Do0result = get_request(link).expect("Error");
        let mut shorted: String = "https://do0.ir/".to_string();
        shorted.push_str(&do0_answer.short);
        println!("Short link is {}", shorted);
        copy_to_clipboard(shorted);
    } else {
        println!("please input the valid link...and try again.");
    }
}
