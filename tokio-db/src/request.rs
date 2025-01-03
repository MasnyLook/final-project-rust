use serde::{Serialize, Deserialize};
use crate::models::AuthenticationToken;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
} // TODO: impl new method

#[derive(Serialize, Deserialize)]
pub struct ServerTokenResponse {
    pub is_authenticated: bool, 
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountData {
    pub name: String,
    pub password: String, // change later to private, get_method
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResultData {
    pub token: AuthenticationToken,
    pub score_time: i32, // can't put u32 in postgresql database
    pub score_moves: i32,
    pub game_type: String,
    pub timestamp: String,
}