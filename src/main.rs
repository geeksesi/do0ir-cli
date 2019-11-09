extern crate reqwest;

use std::collections::HashMap;
use std::env;

fn get_request() -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let params = [("link", "http://google.com")];
    let resp = client
        .post("https://do0.ir/post/sD9qHKZaUC7g/2.5/code")
        .form(&params)
        .send()?
        .json()?;
    println!("{:#?}", resp);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    get_request().expect("could not read file");
}
