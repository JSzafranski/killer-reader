pub mod reader;
#[allow(dead_code)]
pub mod types;

use anyhow::Result;
use reqwest::Client;
use types::MatchCollection;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let req = client
        .get("https://website-backend.w3champions.com/api/matches/search")
        .query(&[("gateWay", "20")])
        .query(&[("season", "14")])
        .query(&[("gameMode", "1")])
        .query(&[("playerId", "LEON#23655")])
        .build()?;
    assert_eq!(req.url().as_ref(), "https://website-backend.w3champions.com/api/matches/search?gateWay=20&season=14&gameMode=1&playerId=LEON%2323655");

    let body = reqwest::get("https://website-backend.w3champions.com/api/matches/search?gateWay=20&season=14&gameMode=1&playerId=LEON#23655")
    .await?
    .text()
    .await?;

    let result: MatchCollection = serde_json::from_str(&body).expect("Failed to deserialize JSON");
    println!("{:#?}", result);
    Ok(())
}
