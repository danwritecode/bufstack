use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    /// Create a new database configuration from environment variables
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://./data.db".to_string());

        Self { database_url }
    }

    /// Create a new database configuration with a custom URL
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite://./data.db".to_string(),
        }
    }
}
