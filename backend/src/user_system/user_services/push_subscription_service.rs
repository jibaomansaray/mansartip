use crate::{
    app::app_helpers::app_error_helper::AppError,
    user_system::{user_entities::{PushSubscriptionEntity, UserEntity}, user_helpers::create_push_subscription_failed_error::CreatePushSubscriptionFailedError},
};

use super::push_subscription_repo_service::{
    PushSubscriptionRepoService, PushSubscriptionRepoServiceTrait,
};

pub struct PushSubscriptionService<R = PushSubscriptionRepoService> {
    repo: R,
}

impl<R> PushSubscriptionService<R>
where
    R: PushSubscriptionRepoServiceTrait,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all_by_user(&self, user: &UserEntity) -> Option<Vec<PushSubscriptionEntity>> {
        self.repo.find_all_by_user(user).await
    }

    pub async fn get_by_internal_id(
        &self,
        internal_id: &str,
        user: &UserEntity,
    ) -> Option<PushSubscriptionEntity> {
        self.repo
            .find_one_by_internal_id_and_user(internal_id, user)
            .await
    }

    pub async fn subscribe(
        &self,
        subscription: &str,
        user_id: u64,
    ) -> Result<PushSubscriptionEntity, AppError> {
        let entity = PushSubscriptionEntity::new(user_id, subscription);
        dbg!(&entity);
        self.repo.insert_push_subscription(entity).await
    }

    pub async fn unsubscribe(
        &self,
        internal_id: &str,
        user: &UserEntity,
    ) -> Result<PushSubscriptionEntity, AppError> {
        match self
            .get_by_internal_id(internal_id, user)
            .await
        {
            Some(entity) => {
             match self.repo.delete_push_subscription(&entity).await {
                Ok(_) => Ok(entity),
                Err(e) => Err(e)
             }

            },
            None => Err(CreatePushSubscriptionFailedError::to_delete_error(Some("push subscription not found"))),
        }
    }
}
