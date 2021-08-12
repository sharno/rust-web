use sqlx::{query_as, PgPool};
use std::{collections::HashMap, convert::Infallible};
use warp::{reject, Rejection, Reply};

use crate::data::MyObject;

pub async fn example1(db: PgPool, p: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let temp: sqlx::Result<(u32,)> = query_as("select 1").fetch_one(&db).await;
    match (temp, p.get("key")) {
        (Ok(i), Some(_)) => Ok(warp::reply::json(&i.0.to_string())),
        (_, None) => Ok(warp::reply::json(&"No \"key\" param in query.")),
        (Err(_), _) => Err(reject()),
    }
}

pub async fn example2(p: MyObject) -> Result<impl Reply, Infallible> {
    log::info!("Hiii");
    Ok(warp::reply::json(&format!(
        "key1 = {}, key2 = {}",
        p.key1, p.key2
    )))
}
