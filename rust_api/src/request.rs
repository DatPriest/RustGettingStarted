pub mod request {
    use std::{fs, path::Path};

    use crate::structs::{RkiWrapper, WeatherWrapper};
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use rocket::figment::providers::Toml;

    pub async fn get_weather_data() -> Result<WeatherWrapper, Box<dyn std::error::Error>> {
        let parameter = load_config();
        tracing::error!(?parameter);
        let weather_forecast = "https://api.openweathermap.org/data/2.5/weather?q=Nordenham&appid=0d754cce3d011e0dcd57dd4ae2f7a414";

        let client = reqwest::Client::new();

        let mut headers: HeaderMap = HeaderMap::new();
        let content_type: HeaderValue = HeaderValue::from_str("text/json").unwrap();
        headers.insert(CONTENT_TYPE, content_type);
        let req = client.get(weather_forecast).headers(headers);

        let data_resp: WeatherWrapper = req.send().await?.json().await?;
        Ok(data_resp)
    }

    pub fn load_config(path: &Path) -> String {
        let toml: str = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let bytes = include_bytes!("../config.toml").to_vec();
        //for byte in bytes {
        //let data = std::str::from_utf8(&byte).unwrap();
        //tracing::error!(?data);
        //}
        let x = String::from_utf8(bytes).unwrap().to_string();
        return x;
        //std::str::from_utf8(bytes).unwrap()
    }

    pub async fn get_rki_data() -> Result<RkiWrapper, Box<dyn std::error::Error>> {
        let history_data = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";

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
}

mod test {

    #[test]
    fn test_json() {
        let a = r#"{
            "objectIdFieldName":"ObjectId",
            "uniqueIdField":{
               "name":"ObjectId",
               "isSystemMaintained":true
            },
            "globalIdFieldName":"",
            "fields":[],
            "features":[
               {
                  "attributes":{
                     "AdmUnitId":4011,
                     "BundeslandId":4,
                     "Datum":1586995200000,
                     "AnzFallNeu":0,
                     "AnzFallVortag":44,
                     "AnzFallErkrankung":8,
                     "AnzFallMeldung":38,
                     "KumFall":527,
                     "ObjectId":44062
                    }
                }
            ]  
        }"#;
        let rki: crate::structs::RkiWrapper = serde_json::from_str(&a).unwrap();
        dbg!(rki);
    }
}
