use serde::Deserialize;
use std::fs::File;
use std::io::Read;
#[derive(Debug, Clone, Default, Deserialize)]

pub struct Waypoint {
    pub symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Faction {
    pub symbol: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct System {
    pub symbol: String,
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<Waypoint>,
    pub factions: Vec<Faction>,
}

pub fn parse_json() -> Result<Vec<System>, std::io::Error> {
    let file_path = "./config/systems.json";
    // Open the file in read-only mode
    let file = File::open(file_path);
    match file {
        Ok(mut file) => {
            println!("File opened successfully");
            // Read the file content into a string
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");

            // Deserialize the JSON into a Vec<System>
            let systems: Vec<System> =
                serde_json::from_str(&contents).expect("Failed to parse JSON");
            Ok(systems)
        }
        Err(error) => {
            println!("Failed to open file: {}", error);

            Err(error)
        }
    }
}
