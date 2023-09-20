use crate::types::MatchCollection;
use anyhow::{Ok, Result};

// fn

use reqwest::Client;

async fn get_page(
    client: Client,
    season: i32,
    race: Race,
    game_mode: GameMode,
) -> Result<MatchCollection> {
    let request = client
        .get("https://website-backend.w3champions.com/api/matches/search")
        .query(&[("playerRace", race.race_to_raceid())])
        .query(&[("gateWay", "20")])
        .query(&[("season", season.to_string())])
        .query(&[("gameMode", game_mode.gamemode_to_gamemodeid())])
        .query(&[("playerId", "LEON#23655")])
        .build()?;
    let body = client.execute(request).await?.text().await?;
    let result: MatchCollection = serde_json::from_str(&body).expect("Failed to deserialize JSON");
    Ok(result)
}

enum Race {
    HU,
    UD,
    ORC,
    NE,
    ALL,
    RANDOM,
}

impl Race {
    fn race_to_raceid(&self) -> String {
        match self {
            Race::HU => 1.to_string(),
            Race::UD => 8.to_string(),
            Race::ORC => 2.to_string(),
            Race::NE => 4.to_string(),
            Race::ALL => todo!(),
            Race::RANDOM => 0.to_string(),
        }
    }
}

enum GameMode {
    ONEVSONE,
    TWOVSTWO,
    FOURVSFOUR,
}

impl GameMode {
    fn gamemode_to_gamemodeid(&self) -> String {
        match self {
            GameMode::ONEVSONE => 1.to_string(),
            GameMode::TWOVSTWO => 2.to_string(),
            GameMode::FOURVSFOUR => 3.to_string(),
        }
    }
}
