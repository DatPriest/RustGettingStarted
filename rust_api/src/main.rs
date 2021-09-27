mod request;
mod structs;
#[macro_use] extern crate rocket;
use std::{collections::HashMap};
use chrono::NaiveDateTime;
use request::request::get_weather_data;
use structs::{RkiData, WeatherWrapper};
use crate::request::request::get_rki_data;


// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter("debug")
        .init();

    let metrics = "process_max_fds 1.048576e+06";
    rocket::build()
        .mount("/v1/", routes![get_data, get_weather_metrics])
        .launch()
        .await.expect("This could not be a error");
}

#[get("/rkidata/metrics")]
async fn get_data () -> String {
    
    let mut vec : Vec<RkiData> = Vec::<RkiData>::new(); 
    let mut result = HashMap::new();
    let data = get_rki_data().await;
    let mut test  = "".to_string();

    for feature in &data.unwrap().features {
        result.insert(feature.attributes.object_id, feature.attributes);
        vec.push(feature.attributes);
    }

    for _data in vec {
        test += &_data.as_prometheus_string();
    }
    test
}

#[get("/weather/metrics")]
async fn get_weather_metrics() -> String {
    match get_weather_data().await {
        Ok(mut _result) => {
            format_to_prom_weather_metrics(_result)
        }
        Err(_err) => {
            tracing::info!(?_err);
            "".to_owned()
        } 
    }
}

fn format_to_prom_weather_metrics(map : WeatherWrapper) -> String {
    let mut text = "".to_owned();
    text += "temp ";
    text += &to_celcius(map.main.temp).to_string();

    text += "\ntemp_min ";
    text += &to_celcius(map.main.temp_min).to_string();

    text += "\ntemp_max ";
    text += &to_celcius(map.main.temp_max).to_string();

    text += "\nsea_level ";
    text += &map.main.sea_level.to_string();

    text += "\npressure ";
    text += &map.main.pressure.to_string();

    text += "\nspeed ";
    text += &to_kmh(map.wind.speed).to_string();

    text += "\ndeg ";
    text += &map.wind.deg.to_string();


    //for data in attributeList {
    //    text += &("<br>".to_owned() +  &NaiveDateTime::from_timestamp(data.dt, 0).to_string() + &"</br>".to_owned());
    //}
    text
}

fn to_celcius(float : f32) -> f32 {
    float - 273.15
}

fn to_kmh(float : f32) -> f32 {
    float * 3.6
}

async fn format_to_prom_rkidata(data : HashMap<i32, RkiData>) -> String {
    let mut text = "".to_owned();
    for rki_data in data.into_values() {
        text += &("<br>".to_owned() +  &NaiveDateTime::from_timestamp(rki_data.datum, 0).to_string() + &"</br>".to_owned());
    }
    text
}