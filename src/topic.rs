use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    name: String,
    description: String,
    payload_schema: Value,
}

impl Topic {
    pub fn new() -> Topic {
        todo!("Create a new, empty Topic");
    }

    pub fn from_json(file_path: &str) -> Topic {
        let data = fs::read_to_string(file_path)
            .expect(format!("Unable to read file: {file_path}").as_str());
        let t: Topic = serde_json::from_str(&data).unwrap();
        // TODO: error checking
        t
    }
}
