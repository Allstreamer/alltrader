use color_eyre::eyre::Context;
use reqwest::Url;
use serde::Deserialize;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::utils::Here;

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

pub async fn parse_json() -> color_eyre::Result<Vec<System>> {
    let file_path = "./config/systems.json";
    fs::create_dir_all("./config/").await?;
    // Open the file in read-only mode
    let file = File::open(file_path).await;
    match file {
        Ok(mut file) => {
            println!("File opened successfully");
            // Read the file content into a string
            let mut contents = String::new();

            file.read_to_string(&mut contents).await.wrap_err(format!(
                "{} Failed to read file: {}",
                Here!(),
                file_path
            ))?;
            // In the future we may wanna fall back to downloading
            let parsed_json = serde_json::from_str(&contents)
                .wrap_err(format!("{} Failed to parse file!", Here!()));
            println!("File parsed successfully");
            parsed_json
            // Deserialize the JSON into a Vec<System>
        }
        Err(error) => {
            println!("{} Failed to open file: {}", Here!(), error);
            let download_result = download_file(
                "https://api.spacetraders.io/v2/systems.json",
                "./config/systems.json",
            )
            .await;
            match download_result {
                Ok(_) => {
                    println!("File downloaded successfully");
                    let mut file = File::open(file_path).await.unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).await.wrap_err(format!(
                        "{} Failed to read file: {}",
                        Here!(),
                        file_path
                    ))?;
                    let systems: Vec<System> = serde_json::from_str(&contents)
                        .wrap_err(format!("{} Failed to parse JSON!", Here!()))?;
                    return Ok(systems);
                }
                Err(error) => {
                    println!("Failed to download file: {}", error);
                }
            }
            Err(error.into())
        }
    }
}

async fn download_file(url: &str, file_path: &str) -> color_eyre::Result<()> {
    // Parse the URL
    let url = Url::parse(url)?;

    // Send a GET request to the URL and store the response in a variable
    let response = reqwest::get(url).await?;
    let raw_response_data = response.text().await?;

    // Create a new file with the same name as the downloaded file and open it for writing
    let mut file = File::create(file_path).await?;
    // Copy the contents of the response to the file
    file.write_all(raw_response_data.as_bytes()).await?;

    Ok(())
}
