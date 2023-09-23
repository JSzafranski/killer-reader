#![allow(dead_code)]

pub mod reader;
pub mod types;
pub mod watcher;

use std::{path::PathBuf, thread::sleep, time};

use crate::reader::{GameMode, Race};
use anyhow::Result;
use reqwest::Client;

const WATCHLIST: &str = "./watchlist.txt";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let matches =
        reader::get_matches(&client, 16, Race::UD, GameMode::ONEVSONE, "Happy#2384").await?;
    // println!("{:#?}", matches);
    assert!(!matches.is_empty());

    let watchlist = watcher::read_watchlist(PathBuf::from(WATCHLIST));

    loop {
        let ongoing_matches = watcher::get_ongoing_matches(&client).await?;
        let active_players = watcher::get_active_players(ongoing_matches);
        println!(
            "{:?}",
            watcher::compare_to_watchlist(active_players, watchlist.clone())
        );
        sleep(time::Duration::from_secs(10));
    }

    // Ok(())
}
