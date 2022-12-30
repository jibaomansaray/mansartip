use std::env;
use std::sync::Arc;

use sqlx::mysql::{MySql, MySqlRow, MySqlPoolOptions};
use sqlx::{pool, Pool};


pub type DbPool = pool::Pool<MySql>;
pub type DbRow = MySqlRow;

pub struct AppState {
  pub db_pool: Arc<Pool<MySql>>,
  pub vapid_public_key: String
}

impl  AppState {
   pub async fn get_instance() -> Self {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("mysql://root:dbpassword@db/rs_worldcup"));
      
    let db_pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("could not create database pool");

      Self {
        db_pool: Arc::new(db_pool),
        vapid_public_key: std::env::var("VAPID_PUBLIC_KEY").unwrap_or_else(|_| String::from(""))
      }
   } 
}