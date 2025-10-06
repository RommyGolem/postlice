use sqlx::SqlitePool;
use std::sync::OnceLock;

static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_pool() -> sqlx::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    POOL.set(pool).map_err(|_| sqlx::Error::PoolClosed)?;
    Ok(())
}

pub fn get_pool() -> Option<&'static SqlitePool> {
    POOL.get()
}
