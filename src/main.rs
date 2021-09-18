use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool};

mod data;
mod db;
mod handler;

use crate::data::MyObject;

struct AppState {
    app_name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(t: web::Path<(u32, String)>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}! id:{} from {}", t.0, t.1, app_name)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().expect(".env file not found");
    pretty_env_logger::init();

    log::debug!("connecting to DB");
    std::thread::sleep(std::time::Duration::from_millis(1));
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Couldn't connect to DB");
    log::debug!("connected to DB!");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
