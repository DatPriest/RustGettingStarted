mod request;

use std::any::Any;
use std::{collections::HashMap};
use std::sync::Arc;
use parking_lot::RwLock;
use request::request::get_rki_data;
use warp::{Filter, http, hyper::Response};
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_LENGTH, CONTENT_TYPE};
use std::fs::File;

use serde_json::{Error, Value};
use crate::request::request::RkiData;

// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    let host = [0, 0, 0, 0];
    let port = 3030;
    let metrics = "process_max_fds 1.048576e+06";


    
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());


    let custom = warp::path("v1").and(warp::path("metrics")).map(move || {
        Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .body(metrics.to_string())
    });

    let any = warp::any().map(|| {
        Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .body("The wrong way")
    });

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);



    let routes = add_items.or(get_items).or(custom).or(any);

    warp::serve(routes)
        .run((host, port))
        .await;
}



type Items = HashMap<i32, RkiData>;
#[derive(Serialize, Deserialize)]
struct Id {
    name: String
}

#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Items>>
  }

impl Store {
    fn new() -> Self {
        Store { 
            grocery_list: 
                Arc::new(
                    RwLock::new(
                        Items::new()
                    )
                ), 
        }
    }
}


async fn update_grocery_list(
    item: RkiData,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(0, item);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn get_grocery_list(
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    //let mut result = HashMap::new();
    //let r = store.grocery_list.read();

    let data = get_rki_data().await;
    println!("{:#?}", &data);

    for (k, v) in data.unwrap() {
        result.insert(k, v);
    }

    warp::reply::reply();
    Ok(warp::reply::json(&data
    ))
}


async fn delete_grocery_list_item(
    item: RkiData,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        //store.grocery_list.write().remove(&item);

        
        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}



fn delete_json() -> impl Filter<Extract = (RkiData,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (RkiData,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}