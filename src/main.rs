pub mod reader;
pub mod types;
pub mod watcher;

use std::{path::PathBuf, thread::sleep, time};

use crate::reader::{GameMode, Race};
use anyhow::Result;
use log::trace;
use reqwest::Client;

const WATCHLIST_PATH: &str = "./watchlist.txt";
const API_CALL_FREQUENCY: time::Duration = time::Duration::from_secs(20);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = Client::new();
    let matches =
        reader::get_matches(&client, 16, Race::UD, GameMode::ONEVSONE, "Happy#2384").await?;
    trace!("{:#?}", matches);

    let watchlist = watcher::read_watchlist(PathBuf::from(WATCHLIST_PATH));

    loop {
        let ongoing_matches = watcher::get_ongoing_matches(&client).await?;
        let active_players = watcher::get_active_players(ongoing_matches);
        let active_watched_players = watcher::compare_to_watchlist(active_players, &watchlist);
        for (name, data) in active_watched_players {
            println!("ONLINE {}: {} MMR", name, data.old_mmr);
        }
        sleep(API_CALL_FREQUENCY);
    }
}
