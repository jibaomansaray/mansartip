use std::future::{ready, Ready};

use actix_web::{FromRequest, HttpMessage};

use crate::user::user_entities::UserEntity;

use super::not_authenticated_error::NotAuthenticatedError;

#[derive(Debug)]
pub struct AuthenticatedUser(pub UserEntity);
impl FromRequest for AuthenticatedUser {
    type Error = NotAuthenticatedError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        let result = match extensions.get::<UserEntity>() {
            Some(auth_user) => Ok(AuthenticatedUser(auth_user.clone())),
            None => Err(NotAuthenticatedError::default()),
        };

        ready(result)
    }
}

impl std::ops::Deref for AuthenticatedUser {
    type Target = UserEntity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AuthenticatedUser {
    pub fn into_inner(self) -> UserEntity {
        self.0
    }
}
