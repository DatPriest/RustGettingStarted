use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use warp::{Filter, http, hyper::Response, reply::Html};
use serde::{Serialize, Deserialize};
use reqwest::header::{USER_AGENT, CONTENT_LENGTH, CONTENT_TYPE};


// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

#[tokio::main]
async fn main() {
    let metrics = "teststats EineMilliarde00000000";


    
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let custom = warp::path("v1").and(warp::path("metrics")).map(move || {
        Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .body(metrics.to_string())
    });

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);


    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);



    let routes = add_items.or(get_items).or(delete_item).or(update_item).or(custom);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


type Items = HashMap<i32, Item>;

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    quantity: i32,
    id: i32
}

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
    item: Item,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.id, item);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn get_grocery_list(
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.grocery_list.read();

    
    for (k, v) in r.iter() {
        result.insert(k, v);
    }

    warp::reply::reply();
    Ok(warp::reply::html("Test"
    ))
}

/*
async fn get_response() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply());
    Err(warp::reject())
}*/

async fn delete_grocery_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().remove(&item.id);

        
        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}



fn delete_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}