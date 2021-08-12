use sqlx::{postgres::PgPoolOptions, PgPool};
use std::collections::HashMap;
use warp::Filter;

mod data;
mod db;
mod handler;

use crate::data::MyObject;

type Db = PgPool;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().expect(".env file not found");
    pretty_env_logger::init();

    log::debug!("connecting to DB");
    log::debug!("Hellooooo");
    std::thread::sleep(std::time::Duration::from_millis(1));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await?;

    let example1 = warp::get()
        .and(warp::path("example1"))
        .and(with_db(pool))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::example1);

    let example2 = warp::get()
        .and(warp::path("example2"))
        .and(warp::query::<MyObject>())
        .and_then(handler::example2);

    Ok(warp::serve(example1.or(example2))
        .run(([127, 0, 0, 1], 3030))
        .await)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
