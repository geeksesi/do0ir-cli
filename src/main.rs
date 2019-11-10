extern crate reqwest;
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

fn get_request() -> Result<Do0result, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [("link", "http://google.com")];
    let response: Do0result = client
        .post("https://do0.ir/post/sD9qHKZaUC7g/2.5/code")
        .form(&params)
        .send()?
        .json()?;

    // println!("{:#?}", response);
    // decode_me(resp);
    Ok(response)
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let do0_answer: Do0result = get_request().expect("could not read file");
    // do0_answer.short = 
    // println!("length : {}", do0_answer.short)
    println!("Short link is https://do0.ir/{}", do0_answer.short)
}
