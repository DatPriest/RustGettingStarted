pub mod request {
    use std::{fs, path::Path};

    use crate::structs::WeatherWrapper;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use rocket::figment::providers::Toml;

    pub async fn get_weather_data() -> Result<WeatherWrapper, Box<dyn std::error::Error>> {
        //let parameter = load_config(Path::new("../config.toml"));
        //tracing::error!(?parameter);
        let weather_forecast = "https://api.openweathermap.org/data/2.5/weather?q=Bremen&appid=0d754cce3d011e0dcd57dd4ae2f7a414";

        let client = reqwest::Client::new();

        let mut headers: HeaderMap = HeaderMap::new();
        let content_type: HeaderValue = HeaderValue::from_str("text/json").unwrap();
        headers.insert(CONTENT_TYPE, content_type);
        let req = client.get(weather_forecast).headers(headers);

        let data_resp: WeatherWrapper = req.send().await?.json().await?;
        Ok(data_resp)
    }

    pub fn load_config(path: &Path) -> String {
        tracing::error!(?path);
        let toml: String = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let bytes = include_bytes!("../config.toml").to_vec();
        //for byte in bytes {
        //let data = std::str::from_utf8(&byte).unwrap();
        //tracing::error!(?data);
        //}
        let x = String::from_utf8(bytes).unwrap().to_string();
        return x;
        //std::str::from_utf8(bytes).unwrap()
    }
}
