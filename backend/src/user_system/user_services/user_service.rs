use crate::user_system::user_entities::UserEntity;

use super::user_repo_service::UserRepoService;

pub struct UserService {
    repo: UserRepoService,
}

impl UserService {
    pub fn new(repo: UserRepoService) -> Self {
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

        if email_or_username.contains("@") { // @todo make this more robust. Check for a valid `email` instead
            user = self.repo.find_user_by_email(email_or_username).await;
        } else {
            user = self.repo.find_user_by_username(email_or_username).await;
        }

        if let Some(user) = user {
            if Self::is_password_correct(&user, password) {
                return Some(user);
            }
        }

        return None;
    }

    pub async fn authenticate_by_token(&self, token: &str) -> Option<UserEntity> {
        self.repo.find_user_by_token(token).await
    }

    pub fn is_password_correct(user: &UserEntity, raw_password: &str) -> bool {
        bcrypt::verify(raw_password, &user.password).unwrap_or(false)
    }
}
