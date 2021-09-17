
pub mod request {
use std::{array, collections::HashMap};

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_LENGTH};
use serde::{Serialize, Deserialize};
use serde_json::{Value};

    pub async fn get_rki_data() -> Result<Vec<RkiData>, Box<dyn std::error::Error>> {

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
    
        let resp = req
            .send()
            .await?;
        let body = resp.text().await?;
    
        let data : Value = serde_json::from_str(&body)?;
        let mut rki_array : Vec<RkiData> = Vec::<RkiData>::new();
        let mut count = 0;
        while !data[count].is_null() {
            rki_array.push(RkiData::convert_to_class(&data[count]["attributes"]));
            count += 1;
            println!("Array Nr #{}", count)
        }
        Ok(rki_array)
    }
    
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct RkiData {
        AdmUnitId: i16,
        AnzFallErkrankung: i32,
        AnzFallMeldung: i32,
        AnzFallNeu: i32,
        AnzFallVortag: i32,
        BundeslandId: i8,
        Datum: i64,
        KumFall: i32,
        ObjectId: i32
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
