use std::sync::Arc;

use crate::{app::app_state::DbPool, user_system::user_entities::UserEntity};
use async_trait::async_trait;
use futures::stream::TryStreamExt;

pub struct UserRepoService {
    pool: Arc<DbPool>,
}

#[async_trait]
pub trait UserRepoServiceTrait {
    async fn find_user_by_id(&self, id: i32) -> Option<UserEntity>;
    async fn find_user_by_internal_id(&self, id: &str) -> Option<UserEntity>;
    async fn find_user_by_token(&self, token: &str) -> Option<UserEntity>;
    async fn find_user_by_email(&self, email: &str) -> Option<UserEntity>;
    async fn find_user_by_username(&self, username: &str) -> Option<UserEntity>;
    async fn insert_user(&self, user: UserEntity) -> Option<UserEntity>;
    async fn update_user(&self, user: UserEntity) -> Option<UserEntity>;
    async fn soft_delete_user(&self, id: i32) -> Option<UserEntity>;
    async fn delete_user(&self, id: i32) -> Option<UserEntity>;
}

impl UserRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepoServiceTrait for UserRepoService {
    async fn find_user_by_id(&self, id: i32) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `id` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
        //  rows.try_next().await.expect("clould not find a user by id")
    }

    async fn find_user_by_internal_id(&self, id: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `internalId` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
    }

    async fn find_user_by_token(&self, token: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `token` = ? and `deletedAt` IS NULL";
        let mut rows = sqlx::query(sql)
            .bind(token)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
    }

    async fn find_user_by_email(&self, email: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `email` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(email)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
    }

    async fn find_user_by_username(&self, username: &str) -> Option<UserEntity> {
        let sql = "SELECT * FROM `user` WHERE `username` = ? and `deletedAt` IS NULL";
        let mut rows = sqlx::query(sql)
            .bind(username)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
    }

    async fn insert_user(&self, user: UserEntity) -> Option<UserEntity> {
        let _sql = "INSERT INTO `user` (`internalId`, `role`, `type`, `username`, `email`, `password`, `token`, `data`, `createdAt`, `updatedAt`, `deletedAt`)
VALUES (?, ?, 1, 'username', 'email@email.com', 'password', 'token', NULL, now(), now(), NULL);";
        // let role:u32 = user.role.parse().or_default(1);

        dbg!(&user);

        Some(user)
    }

    async fn update_user(&self, user: UserEntity) -> Option<UserEntity> {
        None
    }

    async fn soft_delete_user(&self, id: i32) -> Option<UserEntity> {
        None
    }

    async fn delete_user(&self, id: i32) -> Option<UserEntity> {
        None
    }
}
