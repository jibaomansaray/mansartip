use crate::user::user_entities::UserEntity;

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

    pub async fn authenticate_by_token(&self, token: &str) -> Option<UserEntity> {
       self.repo.find_user_by_token(token).await
    }
}
