use std::sync::Arc;

use crate::{app::app_state::DbPool, user::user_entities::UserEntity};
use futures::stream::TryStreamExt;

pub struct UserRepoService {
    pool: Arc<DbPool>,
}

impl UserRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_id(&self, id: i32) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `id` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => {
                None
            }
        }
        //  rows.try_next().await.expect("clould not find a user by id")
    }

    pub async fn find_user_by_internal_id(&self, id: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `internalId` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => {
                None
            }
        }
    }

    pub async fn find_user_by_token(&self, token: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `token` = ? and `deletedAt` IS NULL";
        let mut rows = sqlx::query(sql)
            .bind(token)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => {
                None
            }
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `email` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(email)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => {
                None
            }
        }
    }

    pub async fn insert_user(&self, user: UserEntity ) -> Option<UserEntity> {
      None
    }

    pub async fn update_user(&self, user: UserEntity) -> Option<UserEntity> {
      None
    }

    pub async fn soft_delete_user(&self, id: i32) -> Option<UserEntity> {
      None
    }

    pub async fn delete_user(&self, id: i32) -> Option<UserEntity> {
      None
    }

}
