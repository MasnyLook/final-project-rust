use crate::db::Database;
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
// use derive_more::{Display};
use crate::request::{Response, ServerTokenResponse, AccountData};
use crate::models::GameResult;
use crate::request::GameResultData;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

#[get("/hello")]
async fn hello(
    db: Data<Database>,
) -> Result<Json<Response>, actix_web::Error> {
    Ok(Json(Response { success: true }))
}

#[post("/create_account")]
async fn create_account(
    db: Data<Database>,
    request_data: Json<AccountData>
) -> Result<Json<Response>, actix_web::Error> {
    let name = request_data.name.clone();
    let password = request_data.password.clone(); // remove cloning

    match db.create_new_user(&name, &password).await {
        Ok(()) => Ok(Json(Response {success: true})),
        // Err(anyhow::anyhow!("User already exists")) ,
        Err(_) => Ok(Json(Response {success: false}))
    }
}

#[post("/login")]
async fn login(
    db: Data<Database>,
    request_data: Json<AccountData>
) -> Result<Json<ServerTokenResponse>, actix_web::Error> {
    let name = request_data.name.clone();
    let password = request_data.password.clone();

    match db.authenticate_user(&name, &password).await {
        Ok(token) => Ok(Json(ServerTokenResponse {is_authenticated: true, token: token.cookie})),
        Err(_) => Ok(Json(ServerTokenResponse {is_authenticated: false, token: "".to_string()})),
    }
}

#[post("/game_result")]
async fn game_result( // consider name change
    db: Data<Database>,
    game_data: Json<GameResultData>
) -> Result<HttpResponse, actix_web::Error> { // not sure if i want to return sth here
    let token = game_data.token.clone();
    let datetime = DateTime::parse_from_rfc3339(&game_data.timestamp).expect("Invalid date format");
    let system_time = SystemTime::from(datetime);
    let game_result = GameResult::new (
        token.user_name.clone(),
        game_data.score_time.clone(),
        game_data.score_moves.clone(),
        game_data.game_type.clone(),
        system_time
    );
    db.save_game_result(&game_result, &token).await; // void function i guess
    Ok(HttpResponse::Ok().finish())
}

// leaderboard after merge frontend and backend :))
// test fetching result data after that
