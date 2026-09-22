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

    pub fn random_from(&self, category: &str) -> Option<&String> {
        self.0.get(category)?.choose(&mut rand::thread_rng())
    }
}