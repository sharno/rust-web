use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MyObject {
    pub key1: String,
    pub key2: u32,
}
