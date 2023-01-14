use async_trait::async_trait;
use futures::stream::TryStreamExt;
use std::sync::Arc;

use crate::{
    app::app_state::{DbPool, DbRow},
    tipping_system::tipping_entities::scoreboard_entity::ScoreboardEntity,
    user_system::user_entities::UserEntity,
};

#[async_trait]
pub trait ScoreboardRepoTrait {
    async fn entries(&self, year: i32, limit: u32) -> Vec<ScoreboardEntity>;
    async fn find_by_user_internal_id(
        &self,
        user_internal_id: &str,
        year: i32,
    ) -> Option<ScoreboardEntity>;
}

pub struct ScoreboardRepo {
    pool: Arc<DbPool>,
}

impl ScoreboardRepo {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    fn buil_sql(where_clause: &str) -> (String, impl Fn(DbRow) -> ScoreboardEntity) {
        let template = "SELECT `scoreboard`.*, {user_fields} FROM `scoreboard`
         LEFT JOIN `user` as `tipped_user` on `scoreboard`.`userId` = `tipped_user`.`id` ";

        let user_fields = UserEntity::generate_join_fields(Some("tipped_user"));

        let template = template.replace("{user_fields}", &user_fields.fields);

        (
            format!(
                "{} WHERE `tipped_user`.`deletedAt` IS NULL AND {}",
                template, where_clause
            ),
            move |row| {
                let mut entity = ScoreboardEntity::from_row_ref(&row);
                entity.user = user_fields.transform(&row);

                entity
            },
        )
    }
}

#[async_trait]
impl ScoreboardRepoTrait for ScoreboardRepo {
    async fn entries(&self, year: i32, limit: u32) -> Vec<ScoreboardEntity> {
        let condition = " year = ? LIMIT  ?";

        let built = Self::buil_sql(condition);

        let mut collection = Vec::new();

        let mut rows = sqlx::query(&built.0)
            .bind(year)
            .bind(limit)
            .map(built.1)
            .fetch(self.pool.as_ref());

        while let Some(row) = rows
            .try_next()
            .await
            .expect("could not get scoreboard entries")
        {
            collection.push(row);
        }

        collection
    }

    async fn find_by_user_internal_id(
        &self,
        user_internal_id: &str,
        year: i32,
    ) -> Option<ScoreboardEntity> {
        let conditon = "year = ? and `tipped_user`.`internalId` = ?";
        let built = Self::buil_sql(conditon);

        let result = sqlx::query(&built.0)
            .bind(year)
            .bind(user_internal_id)
            .map(built.1)
            .fetch_one(self.pool.as_ref())
            .await;

        result.ok()
    }
}
