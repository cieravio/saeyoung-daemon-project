use serde::Deserialize;
use std::collection::HasMap;
use std::fs;
use rand::seq::SliceRandom;

#[derive(Deserialize, Clone)]
pub struct Dialogues(HashMap<String, Vec<String>>);

impl Dialogues {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("failed to read dialogues.json");
        serde_json::from_str(&data).expect("invalid JSON format")
    }
}