use serde::Serialize;

#[derive(Serialize)]
pub struct WriteGameLevel {
    pub level_desc: String,
}

#[derive(Serialize)]
pub struct WriteGame {
    pub name: String,
    pub creator: String,
    pub levels: Vec<WriteGameLevel>,
}


pub fn write_json(game: &WriteGame) -> String  {
    let json = serde_json::to_string(game).unwrap();
    json
}
