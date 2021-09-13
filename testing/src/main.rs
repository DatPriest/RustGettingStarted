//#[cfg_attr(target_os = "linux", path = "linux.rs")]
//#[cfg_attr(windows, path = "windows.rs")]

// Access Token 168f3f23-82e5-4db7-9d81-747a43261217

use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}