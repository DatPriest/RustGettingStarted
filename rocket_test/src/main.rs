use rocket::tokio::time::{sleep, Duration};
use std::io;
use rocket::tokio::task::spawn_blocking;
#[macro_use] extern crate rocket;


#[rocket::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_ansi(false)
    .with_env_filter("debug")
    .init();

    rocket::build()
        .mount("/api/", routes![no_route, delay, hello, world, blocking_task])
        .launch()
        .await.unwrap();

}


#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/")]
fn no_route() -> String {
    return "You want to specify the path?".to_owned()
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}