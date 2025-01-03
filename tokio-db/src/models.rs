use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)] // unncecessary Debug and all, change to allow comparison
pub struct AuthenticationToken {
    pub user_name: String,
    pub cookie: String,
}

pub struct GameResult {
    pub user_name: String,
    pub score_time: u32,
    pub score_moves: u32,
    pub game_type: String,
    pub timestamp: String, // check later if all atributes must be public
}
