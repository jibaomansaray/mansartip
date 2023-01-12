use std::sync::Arc;

use sqlx::mysql::{MySql, MySqlRow};
use sqlx::{pool, Database, Pool};

pub type DbPool = pool::Pool<MySql>;
pub type DbRow = MySqlRow;

pub struct AppState<DB = MySql>
where
    DB: Database,
{
    pub db_pool: Arc<Pool<DB>>,
    pub vapid_public_key: String,
    pub year: i32,
}

impl<DB> AppState<DB>
where
    DB: Database,
{
    pub async fn get_instance(pool: Arc<Pool<DB>>, year: i32) -> Self {
        Self {
            db_pool: pool,
            vapid_public_key: std::env::var("VAPID_PUBLIC_KEY")
                .unwrap_or_else(|_| String::from("")),
            year,
        }
    }
}
