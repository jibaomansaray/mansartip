use crate::user_system::{
    user_entities::UserEntity,
    user_helpers::{
        create_user_failed_error::CreateUserFailedError,
        update_user_failed_error::UpdateUserFailedError,
    },
};

use super::user_repo_service::{UserRepoService, UserRepoServiceTrait};

pub struct UserService<T = UserRepoService> {
    repo: T,
}

impl<T> UserService<T>
where
    T: UserRepoServiceTrait,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_user_by_id(&self, id: i32) -> Option<UserEntity> {
        // @todo check permission
        self.repo.find_user_by_id(id).await
    }

    pub async fn get_user_by_token(&self, token: &str) -> Option<UserEntity> {
        // @todo check permission
        self.repo.find_user_by_token(token).await
    }

    // @todo return a result?
    pub async fn log_user_in(&self, email_or_username: &str, password: &str) -> Option<UserEntity> {
        let user;

        if email_or_username.contains('@') {
            // @todo make this more robust. Check for a valid `email` instead
            user = self.repo.find_user_by_email(email_or_username).await;
        } else {
            user = self.repo.find_user_by_username(email_or_username).await;
        }

        if let Some(user) = user {
            if Self::is_password_correct(&user, password) {
                return Some(user);
            }
        }

        None
    }

    pub async fn authenticate_by_token(&self, token: &str) -> Option<UserEntity> {
        self.repo.find_user_by_token(token).await
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<UserEntity, CreateUserFailedError> {
        let mut user = UserEntity::default();

        let user_alreay_exist = self
            .repo
            .find_user_by_username_or_email(username, email)
            .await;

        match user_alreay_exist {
            Some(_) => {
                let mut error = CreateUserFailedError::default();
                error.message = "A user already exist with this email or username".to_owned();
                Err(error)
            }
            None => {
                // set received data
                user.username = username.to_owned();
                user.email = email.to_owned();
                user.password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap_or_default();

                user.reset_token();

                self.repo.insert_user(user).await // store the data to a database
            }
        }
    }

    pub async fn delete_my_account(
        &self,
        internal_id: &str,
    ) -> Result<UserEntity, UpdateUserFailedError> {
        self.delete_user(internal_id, false).await
    }

    pub async fn delete_user(
        &self,
        internal_id: &str,
        soft_delete: bool,
    ) -> Result<UserEntity, UpdateUserFailedError> {
        match self.repo.find_user_by_internal_id(internal_id).await {
            Some(user) => {
                if soft_delete {
                    self.repo.soft_delete_user(user).await
                } else {
                    self.repo.delete_user(user).await
                }
            }
            None => {
                let mut error = UpdateUserFailedError::default();
                error.message = "User does not exist".to_owned();
                Err(error)
            }
        }
    }

    pub fn is_password_correct(user: &UserEntity, raw_password: &str) -> bool {
        bcrypt::verify(raw_password, &user.password).unwrap_or(false)
    }
}
