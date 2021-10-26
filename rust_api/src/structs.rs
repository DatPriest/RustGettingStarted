use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherWrapper {
    pub coord: Coordinates,
    pub weather: Vec<WeatherData>,
    pub base: String,
    pub main: MainData,
    pub visibility: i32,
    pub wind: WindData,
    pub clouds: HashMap<String, serde_json::Value>,
    pub dt: i64,
    pub rain: Option<RainData>,
    pub snow: Option<SnowData>,
}

impl WeatherWrapper {
    pub fn new(
        coord: Coordinates,
        weather: Vec<WeatherData>,
        base: String,
        main: MainData,
        visibility: i32,
        wind: WindData,
        clouds: HashMap<String, serde_json::Value>,
        dt: i64,
        rain: Option<RainData>,
        snow: Option<SnowData>,
    ) -> Self {
        Self {
            coord,
            weather,
            base,
            main,
            visibility,
            wind,
            clouds,
            dt,
            rain,
            snow,
        }
    }

    /*fn clone(&self) -> WeatherWrapper {
        return *self;
    }*/

    fn copy(self) -> Self {
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindData {
    pub speed: f32,
    pub deg: f32,
    gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RainData {
    #[serde(rename = "1h")]
    pub h1: Option<f32>,
    #[serde(rename = "3h")]
    pub h3: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnowData {
    #[serde(rename = "1h")]
    pub h1: Option<f32>,
    #[serde(rename = "3h")]
    pub h3: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainData {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherData {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinates {
    lon: f32,
    lat: f32,
}
