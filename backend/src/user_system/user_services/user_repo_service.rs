use std::{collections::binary_heap, sync::Arc};

use crate::{
    app::app_state::DbPool,
    user_system::{
        user_entities::UserEntity,
        user_helpers::{
            create_user_failed_error::CreateUserFailedError,
            update_user_failed_error::UpdateUserFailedError,
        },
    },
};
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
    async fn find_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> Option<UserEntity>;
    async fn insert_user(&self, user: UserEntity) -> Result<UserEntity, CreateUserFailedError>;
    async fn update_user(&self, user: UserEntity) -> Option<UserEntity>;
    async fn soft_delete_user(&self, user: UserEntity)
        -> Result<UserEntity, UpdateUserFailedError>;
    async fn delete_user(&self, user: UserEntity) -> Result<UserEntity, UpdateUserFailedError>;
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

    async fn find_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> Option<UserEntity> {
        let sql =
            "SELECT * FROM `user` WHERE `username` = ? OR `email` = ? and `deletedAt` IS NULL";

        let mut rows = sqlx::query(sql)
            .bind(username)
            .bind(email)
            .map(UserEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(user) => user,
            _ => None,
        }
    }

    async fn insert_user(&self, user: UserEntity) -> Result<UserEntity, CreateUserFailedError> {
        let sql = "INSERT INTO `user` (`internalId`, `role`, `type`, `username`, `email`, `password`, `token`, `data`, `createdAt`)
VALUES (?, ?, ?, ?, ?, ?, ?, NULL, now());";
        // let role:u32 = user.role.parse().or_default(1);
        let count = sqlx::query(sql)
            .bind(user.internal_id)
            .bind(u32::from(user.role))
            .bind(u32::from(user.user_type))
            .bind(user.username)
            .bind(user.email.clone())
            .bind(user.password)
            .bind(user.token)
            .execute(self.pool.as_ref())
            .await;

        match count {
            Ok(_) => self.find_user_by_email(&user.email).await.ok_or_else(|| {
                let mut error = CreateUserFailedError::default();
                error.message = "could not find the created user".to_owned();
                error
            }),
            Err(e) => {
                dbg!("error creating user: {:?}", &e);
                let mut error = CreateUserFailedError::default();
                error.message = e.to_string();
                Err(error)
            }
        }
    }

    async fn update_user(&self, user: UserEntity) -> Option<UserEntity> {
        None
    }

    async fn soft_delete_user(
        &self,
        user: UserEntity,
    ) -> Result<UserEntity, UpdateUserFailedError> {
        let sql =
            "UPDATE `user` SET `updatedAt` = now(), `deletedAt` = now() WHERE `internalId` = ?";

        let count = sqlx::query(sql)
            .bind(&user.internal_id)
            .execute(self.pool.as_ref())
            .await;
        match count {
            Ok(_) => Ok(user),
            Err(e) => {
                dbg!("error updating user: {:?}", &e);
                let mut error = UpdateUserFailedError::default();
                error.message = e.to_string();
                Err(error)
            }
        }
    }

    async fn delete_user(&self, user: UserEntity) -> Result<UserEntity, UpdateUserFailedError> {
        let sql = "DELETE from `user` WHERE `internalId` = ?";

        let count = sqlx::query(sql)
            .bind(&user.internal_id)
            .execute(self.pool.as_ref())
            .await;

        match count {
            Ok(_) => Ok(user),
            Err(e) => {
                dbg!("error updating user: {:?}", &e);
                let mut error = UpdateUserFailedError::default();
                error.message = e.to_string();
                Err(error)
            }
        }
    }
}
