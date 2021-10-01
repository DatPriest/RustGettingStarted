use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, USER_AGENT};
use serde::Deserialize;
use serde_json::{Error, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Request MainKeys

    let urlKeyData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_key_data_v/FeatureServer/0/query";

    // Request History Data

    let urlHistoryData = "https://services7.arcgis.com/mOBPykOjAyBO2ZKk/arcgis/rest/services/rki_history_v/FeatureServer/0/query";

    let client = reqwest::Client::new();
    let lk_id = "04011";
    let query = [
        ("outFields", "*"),
        ("f", "json"),
        ("geometry", "false"),
        ("where", &format!("AdmUnitId={}", lk_id)),
    ];

    let mut map = HeaderMap::new();
    map.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    map.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let req = client.get(urlHistoryData).query(&query);
    //println!("{:#?}", req);

    let resp = req.send().await?;
    let body = resp.text().await?;

    let data: Value = serde_json::from_str(&body)?;
    let t = &data["features"];
    let mut rki_array: Vec<RkiData> = Vec::<RkiData>::new();
    let mut count = 0;
    while !t[count].is_null() {
        if count > 5 {
            break;
        }
        rki_array.push(RkiData::convert_to_class(&t[count]["attributes"]));

        count += 1;
        println!("Array Nr #{}", count)
    }
    println!("{:#?}", rki_array);
    Ok(())
}

#[derive(Deserialize, Debug, Copy, Clone)]
struct RkiData {
    AdmUnitId: i16,
    AnzFallErkrankung: i32,
    AnzFallMeldung: i32,
    AnzFallNeu: i32,
    AnzFallVortag: i32,
    BundeslandId: i8,
    Datum: i64,
    KumFall: i32,
    ObjectId: i32,
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
        ObjectId: i32,
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

    fn convert_to_class(dat: &Value) -> RkiData {
        let b = RkiData::new(
            dat["AdmUnitId"].to_string().parse::<i16>().unwrap(),
            dat["AnzFallErkrankung"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallMeldung"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallNeu"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallVortag"].to_string().parse::<i32>().unwrap(),
            dat["BundeslandId"].to_string().parse::<i8>().unwrap(),
            dat["Datum"].to_string().parse::<i64>().unwrap(),
            dat["KumFall"].to_string().parse::<i32>().unwrap(),
            dat["ObjectId"].to_string().parse::<i32>().unwrap(),
        );
        return b;
    }
}

async fn send_data(data: &Value) {
    let client: reqwest::Client = reqwest::Client::new();
    let url = "http://localhost:9090/api/v1/targets";
    let query = [("outFields", "*"), ("f", "json")];

    let req = client.post(url);

    let resp = req.send().await;

    match resp {
        Ok(resp) => {
            println!("{}", resp.status());
            let body = &resp.text().await;
            println!("{:#?}", body)
        }
        Err(e) => {
            println!("{:#?}", e)
        }
    }
}
fn sortData(json: Value) {}

async fn get_data(
    url: &str,
    client: reqwest::Client,
    query: [(&str, &str); 4],
) -> Result<(), Error> {
    let req = client.get(url).query(&query);
    println!("{:#?}", req);
    let resp = req.send().await;
    //let body = resp.text().await;
    Ok(())
}
