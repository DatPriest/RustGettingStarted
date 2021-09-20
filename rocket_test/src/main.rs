#[macro_use] extern crate rocket;

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await;
}