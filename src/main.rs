mod json;

use crate::json::write_json::{WriteGame, WriteGameLevel, write_json};
use crate::json::read_json::read_json_typed;


fn parsed_json() {
    let json = r#"
        {
            "name" : "rust game",
            "creator" : "samyakt",
            "levels" : [
                {
                    "level_desc":"basic level"
                },
                {
                    "level_desc":"medium level"
                },
                {
                    "level_desc":"hard level"
                }
            ]
        }
    "#;

    let parsed = read_json_typed(json);

    println!("Game name is : {}", parsed.name);
    println!("Game creator name : {}", parsed.creator);
    println!("The first level : {}", parsed.levels[0].level_desc);
    println!("The third level : {}", parsed.levels[2].level_desc);
}

fn stringfy_json() {
    let game: WriteGame = WriteGame {
        name: String::from("rust game"),
        creator: String::from("samyakt"),
        levels: vec![
            WriteGameLevel {
                level_desc: String::from("basic level")
            },
            WriteGameLevel {
                level_desc: String::from("medium level")
            },
            WriteGameLevel {
                level_desc: String::from("hard level")
            }
        ]
    };

    let json = write_json(&game);

    println!("The JSON is: {}", json);
}


fn main() {
    // read json
    parsed_json();

    println!("\n");

    // write json
    stringfy_json();
}