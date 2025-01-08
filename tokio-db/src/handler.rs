use crate::db::Database;
use crate::models::{AuthenticationToken, GameResult};
use crate::request::GameResultData;
use crate::request::{AccountData, Response, ServerTokenResponse};
use actix_web::{HttpResponse, get, post, web::Data, web::Json};
use chrono::DateTime;
use std::time::SystemTime;

use crate::websockets;

#[get("/hello")]
async fn hello(_db: Data<Database>) -> Result<Json<Response>, actix_web::Error> {
    Ok(Json(Response { success: true }))
}

#[post("/create_account")]
async fn create_account(
    db: Data<Database>,
    request_data: Json<AccountData>,
) -> Result<Json<Response>, actix_web::Error> {
    let name = &request_data.name;
    let password = &request_data.password;

    match db.create_new_user(name, password).await {
        Ok(()) => Ok(Json(Response { success: true })),
        Err(e) if e.to_string() == "User already exists" => Ok(Json(Response { success: false })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/login")]
async fn login(
    db: Data<Database>,
    request_data: Json<AccountData>,
) -> Result<Json<ServerTokenResponse>, actix_web::Error> {
    let name = &request_data.name;
    let password = &request_data.password;
    println!("name: {}, password: {}", name, password);

    match db.authenticate_user(name, password).await {
        Ok(token) => Ok(Json(ServerTokenResponse {
            is_authenticated: true,
            token: token.cookie,
        })),
        Err(e) if e.to_string() == "Invalid password" => Ok(Json(ServerTokenResponse {
            is_authenticated: false,
            token: "".to_string(),
        })),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/game_result")]
async fn game_result(
    db: Data<Database>,
    clients: Data<websockets::AppState>,
    game_data: Json<GameResultData>,
) -> Result<HttpResponse, actix_web::Error> {
    let token = &game_data.token;
    let datetime = DateTime::parse_from_rfc3339(&game_data.timestamp).expect("Invalid date format");
    let system_time = SystemTime::from(datetime);
    let game_result = GameResult::new(
        token.user_name.clone(),
        game_data.score_time,
        game_data.score_moves,
        game_data.game_type.clone(),
        system_time,
    );
    db.save_game_result(&game_result, token, &clients)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/leaderboard")]
async fn leaderboard(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    let leaderboard = db.get_leaderboard().await;
    match leaderboard {
        Ok(leaderboard) => {
            let response = serde_json::to_string(&leaderboard).unwrap();
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        }
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

#[post("/user_board")]
async fn user_board(
    db: Data<Database>,
    request_data: Json<AuthenticationToken>,
) -> Result<HttpResponse, actix_web::Error> {
    match db.get_user_results(&request_data).await {
        Ok(user_board) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&user_board).unwrap())),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}
