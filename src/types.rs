use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Root {
    matches: Vec<Match>,
    count: i32,
}

#[derive(Debug, Deserialize)]
struct Player {
    race: i32,
    rndRace: Option<i32>,
    oldMmr: i32,
    currentMmr: i32,
    battleTag: String,
    name: String,
    mmrGain: i32,
    won: bool,
    location: Option<String>,
    countryCode: Option<String>,
    country: Option<String>,
    twitch: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Team {
    players: Vec<Player>,
    won: bool,
}

#[derive(Debug, Deserialize)]
struct PlayerServerInfo {
    battleTag: String,
    averagePing: i32,
    currentPing: i32,
}

#[derive(Debug, Deserialize)]
struct ServerInfo {
    provider: String,
    nodeId: i32,
    countryCode: Option<String>,
    location: Option<String>,
    name: String,
    playerServerInfos: Vec<PlayerServerInfo>,
}

#[derive(Debug, Deserialize)]
struct Match {
    map: String,
    mapName: String,
    mapId: i32,
    id: String,
    originalOngoingMatchId: String,
    floMatchId: i32,
    durationInSeconds: i32,
    startTime: String,
    endTime: String,
    gameMode: i32,
    teams: Vec<Team>,
    gateWay: i32,
    season: i32,
    number: Option<i32>,
    serverInfo: ServerInfo,
}
