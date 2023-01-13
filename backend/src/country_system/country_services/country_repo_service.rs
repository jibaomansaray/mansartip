use crate::{app::app_state::DbPool, country_system::country_entities::CountryEntity};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use std::sync::Arc;

#[async_trait]
pub trait CountryRepoServiceTrait {
    async fn all(&self) -> Vec<CountryEntity>;
    async fn find_one_by_id(&self, id: i32) -> Option<CountryEntity>;
    async fn find_one_by_internal_id(&self, id: &str) -> Option<CountryEntity>;
}

pub struct CountryRepoService {
    pool: Arc<DbPool>,
}

impl CountryRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CountryRepoServiceTrait for CountryRepoService {
    async fn all(&self) -> Vec<CountryEntity> {
        let sql = "SELECT * FROM `country` WHERE `deletedAt` is null";

        let mut collection = Vec::new();
        let mut rows = sqlx::query(sql)
            .map(CountryEntity::from)
            .fetch(self.pool.as_ref());

        while let Some(row) = rows.try_next().await.expect("could not get all countries") {
            collection.push(row);
        }

        collection
    }

    async fn find_one_by_id(&self, id: i32) -> Option<CountryEntity> {
        let sql = "SELECT * FROM `country` WHERE `id` = ? AND `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(CountryEntity::from)
            .fetch(self.pool.as_ref());

        rows.try_next()
            .await
            .expect("clould not find a country by id")
    }

    async fn find_one_by_internal_id(&self, id: &str) -> Option<CountryEntity> {
        let sql = "SELECT * FROM `country` WHERE `internalId` = ? AND `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(CountryEntity::from)
            .fetch(self.pool.as_ref());

        rows.try_next()
            .await
            .expect("clould not find a country by id")
    }
}
