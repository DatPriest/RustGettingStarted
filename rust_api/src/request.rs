
pub mod request {
use std::io::IoSlice;
use std::io::IoSliceMut;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::{array, collections::HashMap};

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_LENGTH};
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};
use serde_json::{Value};
use warp::body::json;

    pub async fn get_rki_data() -> Result<RkiWrapper, Box<dyn std::error::Error>> {

        // Request MainKeys
    
        let urlKeyData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query";
        
        // Request History Data
    
        let urlHistoryData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";
    
        let client = reqwest::Client::new();
        let lk_id = "04011"; 
        let query = [("outFields", "*"), ("f", "json"), ("geometry", "false"), ("where", &format!("AdmUnitId={}", lk_id))];
    
        let req = client
            .get(urlHistoryData)
            .query(&query);
    
        let data_resp :RkiWrapper = req
            .send()
            .await?.json().await?;
    
        //let body = data_resp.text().await?;
        //println!("{}", body);
        //let data : Value = serde_json::from_str(&body)?;
        //let mut rki_array : Vec<RkiData> = Vec::<RkiData>::new();
        let mut count = 0;
        /*while !data[count].is_null() {
            rki_array.push(RkiData::convert_to_class(&data[count]["attributes"]));
            count += 1;
            println!("Array Nr #{}", count);
        }
        let mut file : std::fs::File = std::fs::File::create("../data/data.json")?;
        //let data = serde_json::json!(rki_array);
        match file.write(data.to_string().as_bytes()) {
            Ok(us) => {

            },
            Err(err) => {
                tracing::error!(?err);
                let mut file: std::fs::File = std::fs::File::create("../logs/log")?;
                file.write(b"Something crashed")?;
            }
        }*/
        tracing::info!(?data_resp);
        Ok(data_resp)
    }
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct RkiAttributes {
        pub attributes : RkiData

    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RkiWrapper {
        pub features: Vec<RkiAttributes>
    }

    #[derive(Deserialize, Debug, Copy, Clone)]
    pub struct RkiData {
        pub AdmUnitId: i16,
        pub AnzFallErkrankung: i32,
        pub AnzFallMeldung: i32,
        pub AnzFallNeu: i32,
        pub AnzFallVortag: i32,
        pub BundeslandId: i8,
        pub Datum: i64,
        pub KumFall: i32,
        pub ObjectId: i32
    }

impl Serialize for RkiData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("RkiData", 3)?;
        state.serialize_field("AdmUnitId", &self.AdmUnitId)?;
        state.serialize_field("AnzFallErkrankung", &self.AnzFallErkrankung)?;
        state.serialize_field("AnzFallMeldung", &self.AnzFallMeldung)?;
        state.serialize_field("AnzFallNeu", &self.AnzFallNeu)?;
        state.serialize_field("AnzFallVortag", &self.AnzFallVortag)?;
        state.serialize_field("BundeslandId", &self.BundeslandId)?;
        state.serialize_field("Datum", &self.Datum)?;
        state.serialize_field("KumFall", &self.KumFall)?;
        state.serialize_field("ObjectId", &self.ObjectId)?;
        state.end()
    }
}
    
    impl RkiData {
        fn new(    
            AdmUnitId: i16,
            AnzFallErkrankung: i32,
            AnzFallMeldung: i32,
            AnzFallNeu: i32,
            AnzFallVortag: i32,
            BundeslandId: i8,
            Datum: i64,
            KumFall: i32,
            ObjectId: i32
        ) -> Self {
            RkiData {
                AdmUnitId,
                AnzFallErkrankung,
                AnzFallMeldung,
                AnzFallNeu,
                AnzFallVortag,
                BundeslandId,
                Datum,
                KumFall,
                ObjectId,
            }
        }
    
        fn convert_to_class(dat : &Value) -> RkiData {
            return RkiData::new(
                dat["AdmUnitId"].to_string().parse::<i16>().unwrap(), 
                dat["AnzFallErkrankung"].to_string().parse::<i32>().unwrap(), 
                dat["AnzFallMeldung"].to_string().parse::<i32>().unwrap(), 
                dat["AnzFallNeu"].to_string().parse::<i32>().unwrap(), 
                dat["AnzFallVortag"].to_string().parse::<i32>().unwrap(), 
                dat["BundeslandId"].to_string().parse::<i8>().unwrap(), 
                dat["Datum"].to_string().parse::<i64>().unwrap(), 
                dat["KumFall"].to_string().parse::<i32>().unwrap(), 
                dat["ObjectId"].to_string().parse::<i32>().unwrap()
            );
        }
    }

}

mod test{
    use super::request::RkiWrapper;

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
        let rki : RkiWrapper = serde_json::from_str(&a).unwrap();
        dbg!(rki);
    }
}
