use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::var("RUST_LOG").unwrap_or_else(|_| env::set_var("RUST_LOG", "info"));
    env_logger::init();

    let server = env::var("SERVER").unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    HttpServer::new(|| {
        App::new().configure(config)
    })
    .bind(server)?
    .run()
    .await
}

