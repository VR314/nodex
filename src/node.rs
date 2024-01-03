use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    project: String,
    name: String,
    language: String,
    command: String,
    runtime_args: Vec<String>,
    init_port: i32,
}

impl Node {
    pub fn from_json(file_path: &str) -> Node {
        // BufReader not used because Node files are typically small
        let data = fs::read_to_string(file_path)
            .expect(format!("Unable to read file: {file_path}").as_str());
        let n: Node = serde_json::from_str(&data).unwrap();
        // TODO: error checking
        n
    }
}
