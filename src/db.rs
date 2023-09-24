use serde::{Deserialize, Serialize};

use std::fs;

const DB_PATH: &str = "./data/data-base.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    name: String,
    url: String,
    online: bool
}

pub fn load_database() -> Vec<Channel> {
    let database_string: String = fs::read_to_string(DB_PATH)
        .expect("Unable to read file");

    let database: Vec<Channel> = serde_json::from_str(&database_string)
        .expect("JSON does not have correct format.");

    return database;
}