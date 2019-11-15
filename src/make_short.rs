extern crate reqwest;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Do0result {
    pub success: bool,
    #[serde(deserialize_with = "parse_null")]
    pub error: String,
    #[serde(default = "default_resource")]
    pub address: String,
    #[serde(default = "default_resource")]
    pub short: String,
}

fn default_resource() -> String {
    "null".to_string()
}

fn parse_null<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("null".to_string()))
}

pub fn get_request(link: String) -> Result<Do0result, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [("link", link)];
    let response: Do0result = client
        .post("https://do0.ir/post/sD9qHKZaUC7g/2.5/code")
        .form(&params)
        .send()?
        .json()?;

    Ok(response)
}
