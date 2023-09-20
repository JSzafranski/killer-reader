pub mod reader;
pub mod types;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://website-backend.w3champions.com/api/matches/search?race=1&gateWay=20&season=14&gameMode=1&playerId=LEON#23655")
    .await?
    .text()
    .await?;
    println!("{body}");
    Ok(())
}
