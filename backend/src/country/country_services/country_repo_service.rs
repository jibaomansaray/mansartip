use std::sync::Arc;

use futures::stream::TryStreamExt;
use crate::{app::app_state::DbPool, country::country_entities::Country};

pub struct CountryRepoService {
    pool: Arc<DbPool>,
}

impl CountryRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    pub async fn all(&self) -> Vec<Country> {
        let sql = "SELECT * FROM country WHERE `deletedAt` is null";

        let mut collection = Vec::new();
        let mut rows = sqlx::query(sql)
            .map(Country::from)
            .fetch(self.pool.as_ref());

        while let Some(row) = rows.try_next().await.expect("could not get all countries") {
            collection.push(row);
        }

        collection
    }

    pub async fn find_one_by_id(&self, id: i32) -> Option<Country> {
        let sql = "SELECT * FROM country WHERE `id` = ? AND `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(Country::from)
            .fetch(self.pool.as_ref());

        rows.try_next().await.expect("clould not find a country by id")
    }
}
