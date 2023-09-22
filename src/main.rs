#![allow(dead_code)]

pub mod reader;
pub mod types;

use crate::reader::{GameMode, Race};
use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    // let page =
    //     reader::get_page(&client, 14, &Race::HU, &GameMode::ONEVSONE, "Leon#23655", 0).await?;
    // println!("{:#?}", page);
    let matches =
        reader::get_matches(&client, 14, Race::HU, GameMode::ONEVSONE, "Leon#23655").await?;
    // println!("{:#?}", matches);

    Ok(())
}
