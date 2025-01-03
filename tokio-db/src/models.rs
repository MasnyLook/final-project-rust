use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)] // unncecessary Debug and all, change to allow comparison
pub struct AuthenticationToken {
    pub user_name: String,
    pub cookie: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)] // unncecessary Debug and all, change to allow comparison
pub struct GameResult {
    pub user_name: String,
    pub score_time: i32, // can't put u32 in postgresql database
    pub score_moves: i32,
    pub game_type: String,
    pub timestamp: SystemTime, // check later if all atributes must be public
}

impl GameResult {
    pub fn new(user_name: String, score_time: i32, score_moves: i32, game_type: String, timestamp: SystemTime) -> GameResult {
        GameResult {
            user_name,
            score_time,
            score_moves,
            game_type,
            timestamp,
        }
    }
}
