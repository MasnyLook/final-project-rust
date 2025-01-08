pub fn create_connection_string() -> String {
    let username = std::env::var("POSTGRES_USER").unwrap_or("".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or("".to_string());
    let host = std::env::var("POSTGRES_HOST").unwrap_or("".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or("".to_string());
    let database = std::env::var("POSTGRES_DB").unwrap_or("".to_string());

    format!(
        "postgresql://{}:{}@{}:{}/{}",
        username, password, host, port, database
    )
}