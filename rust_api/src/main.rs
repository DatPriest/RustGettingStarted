mod config;
mod request;
mod structs;
#[macro_use]
extern crate rocket;
use argh::FromArgs;
use request::request::get_weather_data;
use rocket::figment::Figment;
use std::collections::HashMap;
use structs::WeatherWrapper;

///
#[derive(FromArgs, Debug)]
struct Params {
    /// optional config override param
    #[argh(option, short = 'c', long = "config")]
    config: Option<String>,
    /// request the version of the software
    #[argh(switch)]
    version: bool,
}

// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    let params: Params = argh::from_env();
    if params.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let cfg = config::load(params.config).expect("could not load config");
    let mut figment = rocket::Config::figment()
        .merge(("address", cfg.host))
        .merge(("port", cfg.port));

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter("debug")
        .init();

    rocket::custom(figment)
        .mount("/v1/", routes![get_weather_metrics])
        .launch()
        .await
        .expect("This could not be a error");
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

    if let Some(v) = wrapper.main.sea_level {
        text += "\nsea_level ";
        text += &v.to_string();
    }

    text += "\npressure ";
    text += &wrapper.main.pressure.to_string();

    text += "\nspeed ";
    text += &to_kmh(wrapper.wind.speed).to_string();

    text += "\ndeg ";
    text += &wrapper.wind.deg.to_string();

    text += "\nrainvolume ";
    if let Some(data) = &wrapper.rain {
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
        if let Some(v) = data.h3 {
            text += &v.to_string();
        }
    } else {
        text += "0";
    }

    text += "\nsnowvolume ";
    if let Some(data) = &wrapper.snow {
        if let Some(v) = data.h1 {
            text += &v.to_string();
        }
        if let Some(v) = data.h3 {
            text += &v.to_string();
        }
    } else {
        text += "0";
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
