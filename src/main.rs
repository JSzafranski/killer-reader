#![allow(dead_code)]

pub mod reader;
pub mod types;

use crate::reader::GameMode;
use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let page = reader::get_page(&client, 14, GameMode::ONEVSONE, "LEON#23655").await?;
    println!("{:#?}", page);
    Ok(())
}
