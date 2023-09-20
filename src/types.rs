use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    matches: Vec<Match>,
    count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    map: String,
    map_name: String,
    map_id: i32,
    id: String,
    #[serde(rename = "original-ongoing-match-id")]
    original_ongoing_match_id: String,
    flo_match_id: i32,
    duration_in_seconds: i32,
    start_time: String,
    end_time: String,
    game_mode: i32,
    teams: Vec<Team>,
    gate_way: i32,
    season: i32,
    number: Option<i32>,
    server_info: ServerInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    players: Vec<Player>,
    won: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    race: i32,
    rnd_race: Option<i32>,
    old_mmr: i32,
    current_mmr: i32,
    battle_tag: String,
    name: String,
    mmr_gain: i32,
    won: bool,
    location: Option<String>,
    country_code: Option<String>,
    country: Option<String>,
    twitch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    provider: String,
    node_id: i32,
    country_code: Option<String>,
    location: Option<String>,
    name: String,
    player_server_infos: Vec<PlayerServerInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerServerInfo {
    battle_tag: String,
    average_ping: i32,
    current_ping: i32,
}
