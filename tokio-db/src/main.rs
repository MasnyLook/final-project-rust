mod models;
mod db;
mod handler;
mod request;

use tokio_postgres::{NoTls, Client};
use anyhow::{Context, Error};
use chrono::{DateTime, Utc};

use crate::models::{AuthenticationToken, GameResult};
use crate::db::Database;
use crate::handler::{
    create_account,
    login,
};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use env_logger::Env;
use std::time::SystemTime;

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

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            name TEXT PRIMARY KEY,
            password TEXT NOT NULL
        )
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS scores (
            id SERIAL PRIMARY KEY,
            user_name TEXT NOT NULL,
            score_time INTEGER NOT NULL,
            score_moves INTEGER NOT NULL,
            game_type TEXT NOT NULL,
            timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_name) REFERENCES users (name)
        )
    ").await?;

    Ok(client)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    // Połącz się z bazą danych
    let client = setup_database().await?;
    let db = Database::new(client);
    let db_data = Data::new(
        db
    );
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            .service(create_account)
            .service(login)
            .service(handler::game_result)
    })
    .bind(("127.0.0.1", 8006))?
    .run()
    .await?;

    Ok(())
}


 // // Wstaw dane
    // client.execute(
    //     "INSERT INTO users (name, email) VALUES ($1, $2)",
    //     &[&"John Doe", &"john.doe@example.com"],
    // ).await?;

    // // Wykonaj zapytanie
    // for row in client.query("SELECT id, name, email FROM users", &[]).await? {
    //     let id: i32 = row.get(0);
    //     let name: &str = row.get(1);
    //     let email: &str = row.get(2);

    //     println!("Found user: {} - {} - {}", id, name, email);
    // }



    // // create game result
    // let datetime_str = "2021-06-01T00:00:00Z";
    // let datetime = DateTime::parse_from_rfc3339(datetime_str).expect("Invalid date format");
    // let system_time = SystemTime::from(datetime);
    // let game_result = GameResult::new(
    //     "pablo".to_string(),
    //     100,
    //     100,
    //     "test".to_string(),
    //     system_time
    // );

    // // save game result
    // client.execute(
    //     "INSERT INTO scores (user_name, score_time, score_moves, game_type, timestamp) 
    //     VALUES ($1, $2, $3, $4, $5)",
    //     &[&game_result.user_name, &game_result.score_time, &game_result.score_moves, &game_result.game_type, &game_result.timestamp],
    // ).await?;