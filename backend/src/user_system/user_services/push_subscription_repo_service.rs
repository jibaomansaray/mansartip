use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::TryStreamExt;

use crate::{
    app::{app_helpers::app_error_helper::AppError, app_state::DbPool},
    user_system::{
        user_entities::{PushSubscriptionEntity, UserEntity},
        user_helpers::create_push_subscription_failed_error::CreatePushSubscriptionFailedError,
    },
};

#[async_trait]
pub trait PushSubscriptionRepoServiceTrait {
    async fn find_all_by_user(&self, user: &UserEntity) -> Option<Vec<PushSubscriptionEntity>>;
    async fn find_push_subscription_by_id(&self, id: u64) -> Option<PushSubscriptionEntity>;
    async fn find_one_by_internal_id_and_user(&self, internal_id: &str, user: &UserEntity) -> Option<PushSubscriptionEntity>;
    async fn insert_push_subscription(
        &self,
        subscription: PushSubscriptionEntity,
    ) -> Result<PushSubscriptionEntity, AppError>;
    async fn delete_push_subscription(&self, entity: &PushSubscriptionEntity) -> Result<bool, AppError>;
}

pub struct PushSubscriptionRepoService {
    pool: Arc<DbPool>,
}

impl PushSubscriptionRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PushSubscriptionRepoServiceTrait for PushSubscriptionRepoService {
    async fn find_all_by_user(&self, user: &UserEntity) -> Option<Vec<PushSubscriptionEntity>> {
        let sql = "SELECT * FROM push_subscription WHERE `userId` = ? and `deletedAt` IS NULL";
        let mut entities = Vec::new();

        let mut rows = sqlx::query(sql)
            .bind(&user.internal_id)
            .map(PushSubscriptionEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(entity) => {
                if let Some(e) = entity {
                    entities.push(e);
                }
            },
            _ => (),
        }

        Some(entities)
    }

    async fn find_push_subscription_by_id(&self, id: u64) -> Option<PushSubscriptionEntity> {
        let sql = "SELECT * FROM push_subscription WHERE `id` = ? and `deletedAt` IS NULL";
        let mut rows = sqlx::query(sql)
            .bind(id)
            .map(PushSubscriptionEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(entity) => entity,
            _ => None,
        }
    }


    async fn find_one_by_internal_id_and_user(&self, internal_id: &str, user: &UserEntity) -> Option<PushSubscriptionEntity> {
        let sql = "SELECT * FROM push_subscription WHERE `internalId` = ? and `userId` = ? and `deletedAt` IS NULL";
        let mut rows = sqlx::query(sql)
            .bind(internal_id)
            .bind(user.id)
            .map(PushSubscriptionEntity::from)
            .fetch(self.pool.as_ref());

        match rows.try_next().await {
            Ok(entity) => entity,
            _ => None,
        }
    }

    async fn insert_push_subscription(
        &self,
        entity: PushSubscriptionEntity,
    ) -> Result<PushSubscriptionEntity, AppError> {
        let sql =
            "INSERT INTO `push_subscription` (`internalId`, `userId`, `subscription`, `createdAt`)
VALUES (?, ?, ?, now())";

        let count = sqlx::query(sql)
            .bind(entity.internal_id)
            .bind(entity.user_id)
            .bind(entity.subscription)
            .execute(self.pool.as_ref())
            .await;

        match count {
            Ok(result) => {
                self.find_push_subscription_by_id(result.last_insert_id())
                    .await.ok_or_else(|| CreatePushSubscriptionFailedError::new(None))
            }
            Err(e) => {
                dbg!("error creating push subscripton record: {:?}", &e);
                let error = CreatePushSubscriptionFailedError::new(Some(&e.to_string()));
                Err(error)
            }
        }
    }

    async fn delete_push_subscription(&self, entity: &PushSubscriptionEntity) -> Result<bool, AppError> {
        let sql = "DElETE from `push_subscription` WHERE `internalId` = ?" ;       
        let count = sqlx::query(sql)
            .bind(&entity.internal_id)
            .execute(self.pool.as_ref())
            .await;
        
        match count {
            Ok(_) => Ok(true),
            Err(e) => {
                dbg!("error deleting push subscripton record: {:?}", &e);
                let error = CreatePushSubscriptionFailedError::new(Some(&e.to_string()));
                Err(error)
            }
        }
    }
}
