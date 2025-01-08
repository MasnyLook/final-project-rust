use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthenticationToken {
    pub user_name: String,
    pub cookie: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timestamp {
    pub secs_since_epoch: i64,
    pub nanos_since_epoch: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResult {
    pub user_name: String,
    pub score_time: i32,
    pub score_moves: i32,
    pub game_type: String,
    pub timestamp: Timestamp,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameResultBody {
    pub token: AuthenticationToken,
    pub score_time: i32, // can't put u32 in postgresql database
    pub score_moves: i32,
    pub game_type: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountResponse {
    pub success: bool,
}
