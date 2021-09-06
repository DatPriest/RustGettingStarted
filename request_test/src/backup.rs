use std::{fmt::Result, io::Error, ptr::null};
use serde::{Deserialize, Serialize};
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_LENGTH}};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<> {
    let client = reqwest::Client::new();
    let url = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query";
    let lk_id = "04011"; 
    let query = [("outFields", "*"), ("f", "json"), ("geometry", "false"), ("where", &format!("AdmUnitId={}", lk_id))];

    let mut map = HeaderMap::new();
    map.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    map.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    
    // Request MainKeys


    // Request History Data

    let url = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0";
    let req = client.get(url).query(&query);
    let resp = req.send().await;
    match resp {
        Ok(body) => {
            let text = body.body().await;
            print!("{:#?}", text);
        },
        Err(e) => {

        }
    }
    //let mut x = get_json(url, client, query).await;
    Ok(())

}

async fn get_json(url : &str, client : Client, query : [(&str, &str); 4]) {
    let req = client
        .get(url)
        .query(&query);
    let resp = req.send().await;

    match resp {
        Ok(json) => {
            json.json::<std::collections::HashMap<String, String>>();
        }
        Err(e) => ()
    }   
     /*
    match resp {
        Ok(_body) => (_body),
        Err(e) => {
            println!("Error:{0}", e);
            return String::new();
        },*/
}
