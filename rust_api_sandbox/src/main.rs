extern crate rocket;
use mysql::prelude::Queryable;
use mysql::{from_row, Opts, Pool};
use rocket::Data;
use rocket::{post, routes};
use serde::{Deserialize, Serialize};
// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter("debug")
        .init();

    rocket::build()
        .mount("/v1/", routes![post_player_data])
        .launch()
        .await
        .expect("This could not be a error");
}

#[post("/player/data", format = "application/json", data = "<data>")]
async fn post_player_data<'a>(mut data: Data<'a>) -> String {
    let json = std::str::from_utf8(data.peek(512).await).unwrap();
    let player_data: PlayerData = serde_json::from_str(json)
        .map_err(|e| println!("Error: {}", e.to_string()))
        .unwrap();

    println!("All of the data: {:?}", player_data);
    let url = "mysql://root:root@localhost:3306/sandbox";
    let pool = Pool::new(Opts::from_url(url).unwrap()).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn.query_iter("SELECT steamID, level, experience, inventory FROM playerdata")
        .unwrap()
        .for_each(|row| {
            let r: (String, i32, f32, String) = from_row(row.unwrap());
            println!("{}, {}, {}, {}", r.0, r.1, r.2, r.3);
        });

    "".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerData {
    #[serde(rename(deserialize = "steamId"))]
    steam_id: String,
    level: i32,
    experience: i64,
    inventory: String,
}
