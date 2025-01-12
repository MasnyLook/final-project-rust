use crate::models::{AuthenticationToken, GameResult};
use crate::websockets;
use anyhow::Error;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;
use tokio_postgres::Client;

pub struct Database {
    client: Client,
    tokens: RwLock<HashMap<String, String>>,
    tokens_mutex: Mutex<()>,
}

impl Database {
    pub fn new(client: Client) -> Self {
        Database {
            client,
            tokens: RwLock::new(HashMap::new()),
            tokens_mutex: Mutex::new(()),
        }
    }

    #[allow(dead_code)]
    pub async fn authenticate_user(
        // while logging in
        &self,
        name: &str,
        password: &str,
    ) -> Result<AuthenticationToken, Error> {
        match self
            .client
            .query(
                "SELECT * FROM users WHERE name = ($1) AND password = ($2)",
                &[&name, &password],
            )
            .await
        {
            Ok(rows) => {
                if rows.is_empty() {
                    return Err(anyhow::anyhow!("Invalid password"));
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Database error: {}", e));
            }
        }

        let _guard = self.tokens_mutex.lock().unwrap();
        let token = AuthenticationToken {
            user_name: name.to_string(),
            cookie: self.generate_cookie(),
        };
        self.tokens
            .write()
            .unwrap()
            .insert(name.to_string(), token.cookie.clone());
        Ok(token)
    }

    fn generate_cookie(&self) -> String {
        loop {
            let cookie: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            if !self.tokens.read().unwrap().values().any(|v| v == &cookie) {
                return cookie;
            }
        }
    }

    pub fn authorize_connection(&self, token: &AuthenticationToken) -> bool {
        let _guard = self.tokens_mutex.lock().unwrap();
        match self.tokens.read().unwrap().get(&token.user_name) {
            Some(cookie) => token.cookie.eq(cookie),
            None => false,
        }
    }

    pub async fn create_new_user(&self, name: &str, password: &str) -> Result<(), Error> {
        if !self
            .client
            .query("SELECT * FROM users WHERE name = ($1)", &[&name])
            .await?
            .is_empty()
        {
            return Err(anyhow::anyhow!("User already exists"));
        }
        self.client
            .execute("INSERT INTO users (name, password) VALUES ($1, $2)", &[
                &name, &password,
            ])
            .await?;
        Ok(())
    }

    pub async fn save_game_result(
        &self,
        result: &GameResult,
        token: &AuthenticationToken,
        clients: &websockets::AppState,
    ) -> Result<(), Error> {
        if !self.authorize_connection(token) {
            return Err(anyhow::anyhow!("Unauthorized"));
        }
        match self
            .check_reload_leaderboard(result.score_time, result.score_moves)
            .await
        {
            Ok(true) => clients.send_ping_to_all_clients(),
            Ok(false) => (),
            Err(e) => return Err(e),
        }
        self.client
            .execute(
                "INSERT INTO scores (user_name, score_time, score_moves, game_type, timestamp) 
            VALUES ($1, $2, $3, $4, $5)",
                &[
                    &result.user_name,
                    &result.score_time,
                    &result.score_moves,
                    &result.game_type,
                    &result.timestamp,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_leaderboard(&self) -> Result<Vec<GameResult>, Error> {
        let rows = self
            .client
            .query(
                "SELECT * FROM scores 
            ORDER BY score_moves ASC, score_time ASC, timestamp ASC
            LIMIT 5",
                &[],
            )
            .await?;
        let mut leaderboard = Vec::new();
        for row in rows {
            let game_result = GameResult {
                user_name: row.get(1),
                score_time: row.get(2),
                score_moves: row.get(3),
                game_type: row.get(4),
                timestamp: row.get(5),
            };
            leaderboard.push(game_result);
        }
        Ok(leaderboard)
    }

    #[allow(dead_code)]
    pub async fn get_top10_results(&self, game_type: &str) -> Result<Vec<GameResult>, Error> {
        let rows = self
            .client
            .query(
                "SELECT * FROM scores 
            WHERE game_type = ($1) 
            ORDER BY score_moves ASC, score_time ASC 
            LIMIT 10",
                &[&game_type],
            )
            .await?;
        let mut results = Vec::new();
        for row in rows {
            results.push(GameResult {
                user_name: row.get(1),
                score_time: row.get(2),
                score_moves: row.get(3),
                game_type: row.get(4),
                timestamp: row.get(5),
            });
        }
        Ok(results)
    }

    pub async fn get_user_results(
        &self,
        token: &AuthenticationToken,
    ) -> Result<Vec<GameResult>, Error> {
        if !self.authorize_connection(token) {
            return Err(anyhow::anyhow!("Unauthorized"));
        }
        let rows = self
            .client
            .query(
                "SELECT * FROM scores 
            WHERE user_name = ($1)
            ORDER BY timestamp DESC
            LIMIT 10",
                &[&token.user_name],
            )
            .await?;
        let mut results = Vec::new();
        for row in rows {
            results.push(GameResult {
                user_name: row.get(1),
                score_time: row.get(2),
                score_moves: row.get(3),
                game_type: row.get(4),
                timestamp: row.get(5),
            });
        }
        Ok(results)
    }

    async fn check_reload_leaderboard(
        &self,
        player_time: i32,
        player_moves: i32,
    ) -> Result<bool, Error> {
        let rows = self
            .client
            .query(
                "SELECT score_time, score_moves FROM scores 
            ORDER BY score_moves ASC, score_time ASC, timestamp ASC
            LIMIT 1 OFFSET 4;",
                &[],
            )
            .await?;

        let is_qualified = rows.first().is_none_or(|row| {
            let leaderboard_time: i32 = row.get(0);
            let leaderboard_moves: i32 = row.get(1);
            player_moves < leaderboard_moves
                || (player_moves == leaderboard_moves && player_time <= leaderboard_time)
        });

        Ok(is_qualified)
    }
}
