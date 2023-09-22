#![allow(dead_code)]

pub mod reader;
pub mod types;

use crate::reader::{GameMode, Race};
use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let matches =
        reader::get_matches(&client, 16, Race::ALL, GameMode::ONEVSONE, "Happy#2384").await?;
    // println!("{:#?}", matches);
    assert!(!matches.is_empty());
    Ok(())
}
