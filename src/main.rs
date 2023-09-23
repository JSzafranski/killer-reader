#![allow(dead_code)]

pub mod reader;
pub mod types;
pub mod watcher;

use std::{collections::HashSet, thread::sleep, time};

use crate::reader::{GameMode, Race};
use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let matches =
        reader::get_matches(&client, 16, Race::UD, GameMode::ONEVSONE, "Happy#2384").await?;
    // println!("{:#?}", matches);
    assert!(!matches.is_empty());

    let mut watchlist = HashSet::new();
    watchlist.insert("Lemes".to_string());
    watchlist.insert("Robinson".to_string());
    watchlist.insert("KROLO".to_string());
    watchlist.insert("Nov".to_string());
    watchlist.insert("SaulApeMan".to_string());
    watchlist.insert("Glare".to_string());
    watchlist.insert("黑夜烏鴉".to_string());
    watchlist.insert("Lemei".to_string());
    watchlist.insert("Suelad".to_string());
    watchlist.insert("iNSUPERABLE".to_string());

    loop {
        let ongoing_matches = watcher::get_ongoing_matches(&client).await?;
        let active_players = watcher::get_players(ongoing_matches);
        println!(
            "{:?}",
            watcher::compare_to_watchlist(active_players, watchlist.clone())
        );
        sleep(time::Duration::from_secs(10));
    }

    Ok(())
}
