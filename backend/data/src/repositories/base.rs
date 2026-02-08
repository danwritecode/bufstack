use sqlx::{Pool, Sqlite};

/// Base repository trait for common database operations
pub trait Repository {
    /// Get a reference to the database pool
    fn pool(&self) -> &Pool<Sqlite>;
}
