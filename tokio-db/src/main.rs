mod db;
mod handler;
mod models;
mod request;
mod websockets;

use anyhow::{Context, Error};
use tokio_postgres::{Client, NoTls};

use crate::db::Database;
use crate::handler::{create_account, login};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use std::result::Result;

use actix::prelude::*;
use actix_web::web;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

async fn setup_database() -> Result<Client, Error> {
    // Połącz się z bazą danych
    let (client, connection) =
        tokio_postgres::connect("postgresql://ls448575:dupa123@localhost:5432/db", NoTls)
            .await
            .context("Error connecting to database!")?;

    // Uruchom połączenie w tle
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            name TEXT PRIMARY KEY,
            password TEXT NOT NULL
        )
    ",
        )
        .await?;

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS scores (
            id SERIAL PRIMARY KEY,
            user_name TEXT NOT NULL,
            score_time INTEGER NOT NULL,
            score_moves INTEGER NOT NULL,
            game_type TEXT NOT NULL,
            timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_name) REFERENCES users (name)
        )
    ",
        )
        .await?;

    Ok(client)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    // Połącz się z bazą danych
    let client = setup_database().await?;
    let clients: Arc<Mutex<HashMap<Uuid, Addr<websockets::WebSocketSession>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let db = Database::new(client);
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(db_data.clone())
            .app_data(web::Data::new(websockets::AppState {
                clients: clients.clone(),
            }))
            .route("/ws/", web::get().to(websockets::ws_index))
            .service(create_account)
            .service(login)
            .service(handler::game_result)
            .service(handler::user_board)
            .service(handler::leaderboard)
    })
    .bind(("127.0.0.1", 8006))?
    .run()
    .await?;

    Ok(())
}
