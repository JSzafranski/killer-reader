use crate::types::{Match, MatchCollection};
use anyhow::{Ok, Result};
use reqwest::Client;

pub async fn get_matches(
    client: &Client,
    season: i32,
    race: Race,
    game_mode: GameMode,
    player: &str,
) -> Result<Vec<Match>> {
    let mut matches = Vec::new();
    let mut offset = 0;

    loop {
        let mut match_collection =
            get_page(client, season, &race, &game_mode, player, offset).await?;
        if match_collection.matches.is_empty() {
            assert_eq!(matches.len() as i32, match_collection.count);
            break;
        };
        matches.append(&mut match_collection.matches);
        offset += 100;
    }

    println!("downloaded {} matches", matches.len());

    Ok(matches)
}

pub async fn get_page(
    client: &Client,
    season: i32,
    race: &Race,
    game_mode: &GameMode,
    player: &str,
    offset: i32,
) -> Result<MatchCollection> {
    let request = client
        .get("https://website-backend.w3champions.com/api/matches/search")
        // .query(&[("gateWay", "20")])
        .query(&[("season", season.to_string())])
        .query(&[("gameMode", game_mode.gamemode_to_gamemodeid())])
        .query(&[("playerId", player)])
        .query(&[("playerRace", race.race_to_raceid())])
        .query(&[("offset", offset)])
        .query(&[("pageSize", 100)])
        .build()?;
    println!("{}", request.url());
    let body = client.execute(request).await?.text().await?;
    let result: MatchCollection = serde_json::from_str(&body).expect("Failed to deserialize JSON");
    Ok(result)
}

// based on https://github.com/w3champions/website/blob/master/src/store/types.ts
pub enum Race {
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

pub enum GameMode {
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
