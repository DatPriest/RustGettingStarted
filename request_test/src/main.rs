use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_LENGTH};
use std::fs::File;

use serde_json::{Error, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Request MainKeys

    let urlKeyData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query";

    // Request History Data

    let urlHistoryData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";

    let client = reqwest::Client::new();
    let lk_id = "04011"; 
    let query = [("outFields", "*"), ("f", "json"), ("geometry", "false"), ("where", &format!("AdmUnitId={}", lk_id))];

    let mut map = HeaderMap::new();
    map.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    map.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let req = client
        .get(urlHistoryData)
        .query(&query);
    //println!("{:#?}", req);

    let resp = req
        .send()
        .await?;
    let body = resp.text().await?;


    //let body = get_data(urlHistoryData, client, query).await; 
    
    let json : Value = serde_json::from_str(&body)?;
    //sortData(json);
    send_data(&json).await;
    //println!("{:#?}", json["features"]);
    serde_json::to_writer(&File::create("data/data.json")?, &json["features"]).expect("Something got wrong");
    Ok(())
}


async fn send_data(data : &Value) {
    let client : reqwest::Client = reqwest::Client::new();
    let url ="http://localhost:9090/api/v1/targets";
    let query = [("outFields", "*"), ("f", "json")];

    let req = client
        .post(url);
    
    let resp  = req.send().await;

    match resp{
        Ok(resp) => {
            println!("{}", resp.status());
            let body = &resp.text().await;
            println!("{:#?}", body)
        },
        Err(e) => {
            println!("{:#?}", e)
        }
    }



}
fn sortData(json : Value) {
    
}


async fn get_data(url : &str, client : reqwest::Client, query : [(&str, &str); 4]) -> Result<(), Error> {
    let req = client
        .get(url)
        .query(&query);
    println!("{:#?}", req);
        let resp = req
        .send()
        .await;
    //let body = resp.text().await;
    Ok(())
}
