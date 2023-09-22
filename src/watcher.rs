use std::collections::HashSet;

use anyhow::Result;
use reqwest::Client;

use crate::types::{Match, MatchCollection};

pub async fn get_ongoing_matches(client: &Client) -> Result<Vec<Match>> {
    let mut matches = Vec::new();
    let mut offset = 0;

    loop {
        let mut match_collection = get_page(client, offset).await?;
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

pub async fn get_page(client: &Client, offset: i32) -> Result<MatchCollection> {
    let request = client
        .get("https://website-backend.w3champions.com/api/matches/ongoing")
        .query(&[("offset", offset)])
        .build()?;
    println!("{}", request.url());
    let body = client.execute(request).await?.text().await?;
    let result: MatchCollection = serde_json::from_str(&body).expect("Failed to deserialize JSON");
    Ok(result)
}

pub fn get_players(ongoing_matches: Vec<Match>) -> HashSet<String> {
    let players: HashSet<String> = ongoing_matches
        .iter() // Iterate over the vector of Match structs
        .flat_map(|match_item| {
            match_item.teams.iter().flat_map(|team| {
                team.players.iter().map(|player| {
                    player.name.clone() // Extract the "name" field and clone it
                })
            })
        })
        .collect();
    players
}

pub fn compare_to_watched(
    active_players: HashSet<String>,
    watchlist: HashSet<String>,
) -> HashSet<String> {
    active_players
        .intersection(&watchlist)
        .cloned()
        .collect::<HashSet<String>>()
}
