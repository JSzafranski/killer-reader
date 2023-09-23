use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use log::{debug, warn};
use reqwest::Client;

use crate::types::{Match, MatchCollection, Player};

pub async fn get_ongoing_matches(client: &Client) -> Result<Vec<Match>> {
    let mut matches = Vec::new();
    let mut offset = 0;

    loop {
        let mut match_collection = get_page(client, offset).await?;
        if match_collection.matches.is_empty() {
            if matches.len() as i32 != match_collection.count {
                warn!(
                    "Recorded {} matches, but endpoint reported {}",
                    matches.len(),
                    match_collection.count
                )
            };
            break;
        };
        matches.append(&mut match_collection.matches);
        offset += 100;
    }

    debug!("downloaded {} matches", matches.len());

    Ok(matches)
}

async fn get_page(client: &Client, offset: i32) -> Result<MatchCollection> {
    let request = client
        .get("https://website-backend.w3champions.com/api/matches/ongoing")
        .query(&[("offset", offset)])
        .build()?;
    debug!("{}", request.url());
    let body = client.execute(request).await?.text().await?;
    let result: MatchCollection = serde_json::from_str(&body).expect("Failed to deserialize JSON");
    Ok(result)
}

pub fn get_active_players(ongoing_matches: Vec<Match>) -> HashMap<String, Player> {
    let players: HashMap<String, Player> = ongoing_matches
        .iter()
        .flat_map(|match_entry| {
            match_entry.teams.iter().flat_map(|team| {
                team.players
                    .iter()
                    .map(|player| (player.name.clone(), player.clone()))
            })
        })
        .collect();
    players
}

pub fn compare_to_watchlist(
    active_players: HashMap<String, Player>,
    watchlist: &Vec<String>,
) -> Vec<Player> {
    watchlist
        .iter()
        .filter_map(|name| active_players.get(name).cloned())
        .collect()
}

pub fn read_watchlist(config: PathBuf) -> Vec<String> {
    std::fs::read_to_string(config)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
