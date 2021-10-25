mod request;
mod structs;
#[macro_use]
extern crate rocket;
use crate::request::request::get_rki_data;
use std::collections::HashMap;
use structs::{RkiData};

const URL : &str = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";


// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter("debug")
        .init();

    rocket::build()
        .mount("/v1/", routes![get_data, get_weather_metrics])
        .launch()
        .await
        .expect("This could not be a error");
}

pub async fn get_rki_data() -> Result<RkiWrapper, Box<dyn std::error::Error>> {
    let history_data = URL;
    let client = reqwest::Client::new();
    let lk_id = "04011";
    let query = [
        ("outFields", "*"),
        ("f", "json"),
        ("geometry", "false"),
        ("where", &format!("AdmUnitId={}", lk_id)),
    ];

    let req = client.get(history_data).query(&query);

    let data_resp: RkiWrapper = req.send().await?.json().await?;

    Ok(data_resp)
}

#[get("/rkidata/metrics")]
async fn get_data() -> String {
    let mut vec: Vec<RkiData> = Vec::<RkiData>::new();
    let mut result = HashMap::new();
    let data = get_rki_data().await;
    let mut test = "".to_string();

    data.unwrap().features.iter().for_each(|feature| {
        result.insert(feature.attributes.object_id, feature.attributes);
        vec.push(feature.attributes);
    });

    vec.into_iter().map(|data| {
        test += &data.as_prometheus_string();
        
    });
}
