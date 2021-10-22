mod request;
mod structs;
#[macro_use]
extern crate rocket;
use crate::request::request::get_rki_data;
use request::request::get_weather_data;
use std::collections::HashMap;
use structs::{RkiData, WeatherWrapper};

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

    vec.into_iter().for_each(|x| {
        test += &x.as_prometheus_string();
    });
    test
}

#[get("/weather/metrics")]
async fn get_weather_metrics() -> String {
    let x = "getdata";
    tracing::debug!(?x);
    match get_weather_data().await {
        Ok(mut _result) => format_to_prom_weather_metrics(_result),
        Err(_err) => {
            tracing::info!(?_err);
            _err.to_string()
        }
    }
}

fn format_to_prom_weather_metrics(wrapper: WeatherWrapper) -> String {
    let mut text = "".to_owned();
    text += "temp ";
    text += &to_celcius(wrapper.main.temp).to_string();

    text += "\ntemp_min ";
    text += &to_celcius(wrapper.main.temp_min).to_string();

    text += "\ntemp_max ";
    text += &to_celcius(wrapper.main.temp_max).to_string();

    text += "\nsea_level ";
    text += &wrapper.main.sea_level.to_string();

    text += "\npressure ";
    text += &wrapper.main.pressure.to_string();

    text += "\nspeed ";
    text += &to_kmh(wrapper.wind.speed).to_string();

    text += "\ndeg ";
    text += &wrapper.wind.deg.to_string();

    if let Some(data) = &wrapper.rain {
        text += "\nrainvolume ";
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
    }

    if let Some(data) = &wrapper.snow {
        text += "\nsnowvolume ";
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
    }

    //for data in attributeList {
    //    text += &("<br>".to_owned() +  &NaiveDateTime::from_timestamp(data.dt, 0).to_string() + &"</br>".to_owned());
    //}
    text
}

fn to_celcius(float: f32) -> f32 {
    float - 273.15
}

fn to_kmh(float: f32) -> f32 {
    float * 3.6
}
