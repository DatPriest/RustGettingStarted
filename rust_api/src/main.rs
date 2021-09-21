mod request;
#[macro_use] extern crate rocket;
use std::{collections::HashMap};
use chrono::NaiveDateTime;
use request::request::get_rki_data;

use crate::request::request::RkiData;

// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter("debug")
        .init();

    let metrics = "process_max_fds 1.048576e+06";
    rocket::build()
        .mount("/v1/", routes![get_data])
        .launch()
        .await;
}

#[get("/metrics")]
async fn get_data () -> String {
    let mut result = HashMap::new();
    let mut vec : Vec<RkiData> = Vec::<RkiData>::new(); 
    let data = get_rki_data().await;
    for feature in &data.unwrap().features {
        //println!("{:?}", feature.attributes);
        vec.push(feature.attributes);
        result.insert(feature.attributes.ObjectId, feature.attributes);
    }

    let mut test  = "".to_string();
    for _data in vec {
        test += &_data.to_prometheus_string();
    }
    println!("{}", test);
    return test;
    //return "".to_owned();
}

async fn format_to_prom(data : HashMap<i32, RkiData>) -> String {
    let mut text = "".to_owned();
    for rki_data in data.into_values() {
        text += &("<br>".to_owned() +  &NaiveDateTime::from_timestamp(rki_data.Datum, 0).to_string() + &"</br>".to_owned());
    }
    return text;
}