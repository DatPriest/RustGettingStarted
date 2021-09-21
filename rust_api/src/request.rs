
pub mod request {

use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct};
use serde_json::{Value};

    pub async fn get_rki_data() -> Result<bool, Box<dyn std::error::Error>> {

        // Request MainKeys
    
        let urlKeyData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query";
        
        // Request History Data
    
        let urlHistoryData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";
        let weatherForecast = "api.openweathermap.org/data/2.5/weather?q=Nordenham&appid=0d754cce3d011e0dcd57dd4ae2f7a414";
    
        let client = reqwest::Client::new();
        let lk_id = "04011"; 
        let query = [("outFields", "*"), ("f", "json"), ("geometry", "false"), ("where", &format!("AdmUnitId={}", lk_id))];
    
        let req = client
            .get(weatherForecast);
    
        let data_resp = req
            .send()
            .await?;
    
        println!("{:?}", data_resp.text().await?);
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
        //tracing::info!(?data_resp);
        Ok(true)
    }
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct RkiAttributes {
        pub attributes : RkiData

    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RkiWrapper {
        pub features: Vec<RkiAttributes>
    }

    #[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
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

        pub fn to_prometheus_string(&self) -> String {
            let mut result : String = String::new();
            result += "rkidata{";
            result += "AdmUnitId=\""; result += &self.AdmUnitId.to_string(); result += "\" ";
            result += &self.AnzFallErkrankung.to_string();
            result += &self.AnzFallMeldung.to_string();
            result += &self.AnzFallNeu.to_string();
            result += &self.AnzFallVortag.to_string();
            result += &self.BundeslandId.to_string();
            result += &self.Datum.to_string();
            result += &self.KumFall.to_string();
            result += &self.ObjectId.to_string();
            result += "}\n";
            return result;
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
