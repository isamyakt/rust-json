use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReadGameLevel {
    pub level_desc: String,
}

#[derive(Deserialize)]
pub struct ReadGame {
    pub name: String,
    pub creator: String,
    pub levels: Vec<ReadGameLevel>,
}

pub fn read_json_typed(raw_json: &str) -> ReadGame {
    let parsed = serde_json::from_str(raw_json).unwrap();
    parsed
}