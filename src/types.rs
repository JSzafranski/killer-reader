use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchCollection {
    pub matches: Vec<Match>,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub map: String,
    pub map_name: String,
    pub map_id: i32,
    pub id: String,
    #[serde(rename = "original-ongoing-match-id")]
    pub original_ongoing_match_id: String,
    pub flo_match_id: Option<i32>,
    pub duration_in_seconds: i32,
    pub start_time: String,
    pub end_time: String,
    pub game_mode: i32,
    pub teams: Vec<Team>,
    pub gate_way: i32,
    pub season: i32,
    pub number: Option<i32>,
    pub server_info: ServerInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub players: Vec<Player>,
    pub won: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub race: i32,
    pub rnd_race: Option<i32>,
    pub old_mmr: i32,
    pub current_mmr: i32,
    pub battle_tag: String,
    pub name: String,
    pub mmr_gain: i32,
    pub won: bool,
    pub location: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub twitch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub provider: String,
    pub node_id: i32,
    pub country_code: Option<String>,
    pub location: Option<String>,
    pub name: String,
    pub player_server_infos: Vec<PlayerServerInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerServerInfo {
    pub battle_tag: String,
    pub average_ping: i32,
    pub current_ping: i32,
}
