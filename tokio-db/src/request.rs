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

#[derive(Serialize, Deserialize)]
pub struct GameResultData {
    pub token: AuthenticationToken,
    pub score_time: u32,
    pub score_moves: u32,
    pub game_type: String,
    pub timestamp: String,
}