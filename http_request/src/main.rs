extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    // let mut resp = reqwest::get("http://localhost:8000")?;
    // println!("{:#?}", resp);
    // println!("{:#?}", resp.text()?);
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let client = reqwest::Client::new();
    let mut post_response = client.post("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
        .json(&map)
        .send()?;
    println!("{:#?}", post_response);
    println!("{:#?}", post_response.text());

    Ok(())
}