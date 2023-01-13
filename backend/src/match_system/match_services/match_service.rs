use crate::match_system::match_entities::match_entity::MatchEntity;

use super::match_match_repo_service::{MatchRepoService, MatchRepoServiceTrait};

pub struct MatchService<T = MatchRepoService> {
    repo: T,
}

impl<T> MatchService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T> MatchService<T>
where
    T: MatchRepoServiceTrait,
{
    pub async fn get_today_matches(&self, year: Option<i32>) -> Option<Vec<MatchEntity>> {
        self.repo.todays(year).await
    }

    pub async fn get_all_matches(&self, year: i32) -> Vec<MatchEntity> {
        self.repo.all(year).await
    }
}
